use crate::digest::EvidenceDigest;
use crate::evidence::{
    AGDA_DATA_SNAPSHOT_DIRECTORY_NAME, AGDA_EXPECTED_VERSION, AGDA_PRIMITIVE_RELATIVE_ROOT,
    AGDA_PRIMITIVE_SOURCE_TREE_CONTRACT_ID, AgdaDataDirectoryEvidence,
    AgdaPrimitiveSourceTreeEvidence, AgdaToolchainEvidence, BackendEvidenceError,
    BackendEvidenceManifest, CUBICAL_EXPECTED_COMMIT, CUBICAL_EXPECTED_SOURCE_TREE_DIGEST,
    CUBICAL_LIBRARY_NORMALIZED, CUBICAL_SNAPSHOT_DIRECTORY_NAME, CUBICAL_SOURCE_TREE_CONTRACT_ID,
    CapturedOutput, CheckerInvocationEvidence, CubicalCheckoutEvidence, CubicalSourceFileEvidence,
    CubicalSourceTreeEvidence, MAX_CHECKER_TIMEOUT_MILLIS, ProcessEvidence, SMOKE_FILE_NAME,
    SMOKE_SOURCE, SMOKE_SOURCE_DIGEST, expected_checker_arguments, expected_git_prefix,
    parse_agda_version, source_tree_digest,
};
use command_group::CommandGroup;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;

pub const MAX_CHECKER_TIMEOUT: Duration = Duration::from_millis(MAX_CHECKER_TIMEOUT_MILLIS);
const AUXILIARY_COMMAND_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_CAPTURE_BYTES: u64 = 4 * 1024 * 1024;
const MAX_CUBICAL_TREE_ENTRIES: u64 = 100_000;
const MAX_CUBICAL_SOURCE_FILES: u64 = 20_000;
const MAX_CUBICAL_SOURCE_FILE_BYTES: u64 = 64 * 1024 * 1024;
const MAX_CUBICAL_SOURCE_TREE_BYTES: u64 = 1024 * 1024 * 1024;
const PROCESS_POLL_INTERVAL: Duration = Duration::from_millis(10);
const MAX_SCRATCH_ATTEMPTS: u64 = 1_024;
static SCRATCH_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Executable identities supplied by the trusted runtime configuration.
///
/// The probe never derives these pins from the executables it is about to
/// run. A caller that self-pins observed bytes has established reproducibility
/// only, not tool authenticity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutableDigestPins {
    agda: EvidenceDigest,
    git: EvidenceDigest,
}

impl ExecutableDigestPins {
    pub fn new(agda: EvidenceDigest, git: EvidenceDigest) -> Self {
        Self { agda, git }
    }

    pub fn agda(&self) -> &EvidenceDigest {
        &self.agda
    }

    pub fn git(&self) -> &EvidenceDigest {
        &self.git
    }
}

/// Reviewed imported-tree identities supplied independently of the paths
/// whose bytes the probe materializes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceTreeDigestPins {
    agda_primitive: EvidenceDigest,
    cubical: EvidenceDigest,
}

impl SourceTreeDigestPins {
    pub fn new(agda_primitive: EvidenceDigest, cubical: EvidenceDigest) -> Self {
        Self {
            agda_primitive,
            cubical,
        }
    }

    pub fn agda_primitive(&self) -> &EvidenceDigest {
        &self.agda_primitive
    }

    pub fn cubical(&self) -> &EvidenceDigest {
        &self.cubical
    }
}

/// Filesystem and executable locations for the exact pinned readiness probe.
///
/// All paths must already exist and be absolute. The scratch root must be a
/// directory; the probe creates and removes one private child directory.
/// Executable digests must come from trusted configuration independent of the
/// files being probed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PinnedBackendConfig {
    agda_executable: PathBuf,
    git_executable: PathBuf,
    executable_pins: ExecutableDigestPins,
    source_tree_pins: SourceTreeDigestPins,
    cubical_root: PathBuf,
    scratch_root: PathBuf,
    checker_timeout: Duration,
}

impl PinnedBackendConfig {
    pub fn new(
        agda_executable: impl Into<PathBuf>,
        git_executable: impl Into<PathBuf>,
        executable_pins: ExecutableDigestPins,
        source_tree_pins: SourceTreeDigestPins,
        cubical_root: impl Into<PathBuf>,
        scratch_root: impl Into<PathBuf>,
        checker_timeout: Duration,
    ) -> Self {
        Self {
            agda_executable: agda_executable.into(),
            git_executable: git_executable.into(),
            executable_pins,
            source_tree_pins,
            cubical_root: cubical_root.into(),
            scratch_root: scratch_root.into(),
            checker_timeout,
        }
    }

    pub fn agda_executable(&self) -> &Path {
        &self.agda_executable
    }

    pub fn git_executable(&self) -> &Path {
        &self.git_executable
    }

    pub fn executable_pins(&self) -> &ExecutableDigestPins {
        &self.executable_pins
    }

    pub fn source_tree_pins(&self) -> &SourceTreeDigestPins {
        &self.source_tree_pins
    }

    pub fn cubical_root(&self) -> &Path {
        &self.cubical_root
    }

    pub fn scratch_root(&self) -> &Path {
        &self.scratch_root
    }

    pub fn checker_timeout(&self) -> Duration {
        self.checker_timeout
    }
}

/// Opaque readiness capability minted only by [`probe_pinned_backend`].
///
/// This type deliberately implements neither `Deserialize` nor a public
/// constructor. Its only payload is read-only readiness evidence. It grants no
/// interface for checking arbitrary source or asserting a GF2 theorem.
#[must_use = "a verified backend capability should be consumed by a restricted replay boundary"]
pub struct VerifiedPinnedBackend {
    evidence: BackendEvidenceManifest,
    _seal: private::CapabilitySeal,
}

impl VerifiedPinnedBackend {
    pub fn evidence(&self) -> &BackendEvidenceManifest {
        &self.evidence
    }

    pub fn evidence_digest(&self) -> &EvidenceDigest {
        self.evidence.canonical_digest()
    }
}

impl std::fmt::Debug for VerifiedPinnedBackend {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VerifiedPinnedBackend")
            .field("evidence_digest", &self.evidence_digest())
            .finish_non_exhaustive()
    }
}

mod private {
    pub(super) struct CapabilitySeal;
}

/// Check the exact pinned Agda/Cubical environment using only the internally
/// fixed smoke source.
///
/// The checker and Git processes are invoked directly without a shell. Git
/// top-level and HEAD are checked against fixed pins. Canonical source-tree
/// snapshots and executable/library bytes are checked for mutation across the
/// probe.
pub fn probe_pinned_backend(
    config: &PinnedBackendConfig,
) -> Result<VerifiedPinnedBackend, ProbeError> {
    validate_timeout(config.checker_timeout)?;
    require_absolute(&config.agda_executable, "Agda executable")?;
    require_absolute(&config.git_executable, "Git executable")?;
    require_absolute(&config.cubical_root, "Cubical root")?;
    require_absolute(&config.scratch_root, "scratch root")?;

    let agda_executable = canonical_file(&config.agda_executable, "Agda executable")?;
    let git_executable = canonical_file(&config.git_executable, "Git executable")?;
    let cubical_root = canonical_directory(&config.cubical_root, "Cubical root")?;
    let scratch_root = canonical_directory(&config.scratch_root, "scratch root")?;
    let scratch = ScratchDirectory::create(&scratch_root)?;
    create_new_directory(
        &scratch.path().join(".git-hooks-disabled"),
        "disabled Git hooks directory",
    )?;
    create_new_directory(
        &scratch.path().join(".agda-app"),
        "isolated Agda application directory",
    )?;

    let agda_path_text = path_text(&agda_executable, "Agda executable")?;
    let git_path_text = path_text(&git_executable, "Git executable")?;
    let cubical_root_text = path_text(&cubical_root, "Cubical root")?;
    let scratch_path_text = path_text(scratch.path(), "probe scratch directory")?;

    let agda_digest_before = digest_file(&agda_executable, "Agda executable")?;
    let git_digest_before = digest_file(&git_executable, "Git executable")?;
    require_digest_pin(
        "Agda executable digest",
        config.executable_pins.agda(),
        &agda_digest_before,
    )?;
    require_digest_pin(
        "Git executable digest",
        config.executable_pins.git(),
        &git_digest_before,
    )?;
    let reviewed_source_tree_digest = EvidenceDigest::parse(CUBICAL_EXPECTED_SOURCE_TREE_DIGEST)
        .map_err(|_| {
            ProbeError::InternalContract("reviewed Cubical source-tree digest is malformed")
        })?;
    require_digest_pin(
        "configured Cubical source-tree digest",
        &reviewed_source_tree_digest,
        config.source_tree_pins().cubical(),
    )?;

    let version_arguments = vec!["--version".to_owned()];
    let version_result = run_captured(
        &agda_executable,
        &version_arguments,
        scratch.path(),
        AUXILIARY_COMMAND_TIMEOUT,
        "agda-version",
    )?;
    require_success("Agda version probe", &version_result)?;
    let observed_version =
        parse_agda_version(&version_result.stdout).ok_or(ProbeError::PinMismatch {
            component: "Agda version output",
            expected: format!("first line `Agda version {AGDA_EXPECTED_VERSION}`"),
            actual: version_result.stdout.clone(),
        })?;
    if observed_version != AGDA_EXPECTED_VERSION {
        return Err(ProbeError::PinMismatch {
            component: "Agda version",
            expected: AGDA_EXPECTED_VERSION.to_owned(),
            actual: observed_version,
        });
    }
    let version_evidence = version_result.into_evidence(version_arguments);

    let data_directory_arguments = vec!["--print-agda-data-dir".to_owned()];
    let data_directory_result = run_captured(
        &agda_executable,
        &data_directory_arguments,
        scratch.path(),
        AUXILIARY_COMMAND_TIMEOUT,
        "agda-data-directory",
    )?;
    require_success("Agda data-directory probe", &data_directory_result)?;
    let installed_agda_data_directory = canonical_reported_directory(
        &data_directory_result.stdout,
        "Agda installed data directory",
    )?;
    let installed_agda_data_directory_text = path_text(
        &installed_agda_data_directory,
        "Agda installed data directory",
    )?;
    let installed_primitive_root = canonical_directory(
        &installed_agda_data_directory.join(AGDA_PRIMITIVE_RELATIVE_ROOT),
        "Agda primitive source root",
    )?;
    let snapshot_agda_data_directory = scratch.path().join(AGDA_DATA_SNAPSHOT_DIRECTORY_NAME);
    create_new_directory(
        &snapshot_agda_data_directory,
        "Agda data snapshot directory",
    )?;
    let snapshot_primitive_root = snapshot_agda_data_directory.join(AGDA_PRIMITIVE_RELATIVE_ROOT);
    let primitive_source_tree_before = materialize_source_tree(
        &installed_primitive_root,
        &snapshot_primitive_root,
        config.source_tree_pins().agda_primitive().clone(),
        SourceTreeKind::AgdaPrimitive,
    )?;
    let snapshot_agda_data_directory_text = path_text(
        &snapshot_agda_data_directory,
        "Agda data snapshot directory",
    )?;
    let data_directory_evidence = data_directory_result.into_evidence(data_directory_arguments);

    let cubical_snapshot_root = scratch.path().join(CUBICAL_SNAPSHOT_DIRECTORY_NAME);
    let source_tree_before = materialize_source_tree(
        &cubical_root,
        &cubical_snapshot_root,
        config.source_tree_pins().cubical().clone(),
        SourceTreeKind::Cubical,
    )?;
    let cubical_snapshot_root_text = path_text(&cubical_snapshot_root, "Cubical source snapshot")?;

    let git_prefix = expected_git_prefix(&cubical_root_text);
    let top_arguments = extend_arguments(&git_prefix, &["rev-parse", "--show-toplevel"]);
    let top_result = run_captured(
        &git_executable,
        &top_arguments,
        scratch.path(),
        AUXILIARY_COMMAND_TIMEOUT,
        "git-top-level",
    )?;
    require_success("Git top-level probe", &top_result)?;
    verify_git_top_level(&top_result.stdout, &cubical_root)?;
    let top_evidence = top_result.into_evidence(top_arguments);

    let head_arguments = extend_arguments(&git_prefix, &["rev-parse", "HEAD"]);
    let head_before_result = run_captured(
        &git_executable,
        &head_arguments,
        scratch.path(),
        AUXILIARY_COMMAND_TIMEOUT,
        "git-head-before",
    )?;
    require_success("Git pre-check HEAD probe", &head_before_result)?;
    verify_head(&head_before_result.stdout, "before checker")?;
    let head_before_evidence = head_before_result.into_evidence(head_arguments.clone());

    let library_path = cubical_root.join("cubical.agda-lib");
    let canonical_library_path = canonical_file(&library_path, "Cubical library file")?;
    if canonical_library_path.parent() != Some(cubical_root.as_path()) {
        return Err(ProbeError::PathEscapesRoot {
            component: "Cubical library file",
            root: cubical_root,
            resolved: canonical_library_path,
        });
    }
    let library_bytes_before =
        fs::read(&canonical_library_path).map_err(|source| ProbeError::Io {
            context: format!("read {}", canonical_library_path.display()),
            source,
        })?;
    validate_library_file(&library_bytes_before)?;

    let fixed_source_digest = EvidenceDigest::of_bytes(SMOKE_SOURCE.as_bytes());
    if fixed_source_digest.as_str() != SMOKE_SOURCE_DIGEST {
        return Err(ProbeError::InternalContract(
            "fixed smoke source digest snapshot is inconsistent",
        ));
    }
    let source_path = scratch.path().join(SMOKE_FILE_NAME);
    write_new_file(&source_path, SMOKE_SOURCE.as_bytes(), "fixed smoke source")?;
    let written_source = fs::read(&source_path).map_err(|source| ProbeError::Io {
        context: format!("read back {}", source_path.display()),
        source,
    })?;
    if EvidenceDigest::of_bytes(&written_source) != fixed_source_digest {
        return Err(ProbeError::InternalContract(
            "written smoke source bytes differ from the fixed source",
        ));
    }
    let source_path_text = path_text(&source_path, "fixed smoke source")?;
    let checker_arguments = expected_checker_arguments(
        &scratch_path_text,
        &cubical_snapshot_root_text,
        &source_path_text,
    );
    let checker_result = run_captured_with_agda_data_directory(
        &agda_executable,
        &checker_arguments,
        scratch.path(),
        config.checker_timeout,
        "agda-checker",
        &snapshot_agda_data_directory,
    )?;

    // Recheck the commit before interpreting checker success. Canonical source
    // trees and all snapshotted bytes are rechecked below.
    let head_after_result = run_captured(
        &git_executable,
        &head_arguments,
        scratch.path(),
        AUXILIARY_COMMAND_TIMEOUT,
        "git-head-after",
    )?;
    require_success("Git post-check HEAD probe", &head_after_result)?;
    verify_head(&head_after_result.stdout, "after checker")?;
    let head_after_evidence = head_after_result.into_evidence(head_arguments);

    let library_bytes_after =
        fs::read(&canonical_library_path).map_err(|source| ProbeError::Io {
            context: format!("re-read {}", canonical_library_path.display()),
            source,
        })?;
    if library_bytes_after != library_bytes_before {
        return Err(ProbeError::MutatedDuringProbe("Cubical library file"));
    }
    validate_library_file(&library_bytes_after)?;
    let source_bytes_after = fs::read(&source_path).map_err(|source| ProbeError::Io {
        context: format!("re-read fixed smoke source {}", source_path.display()),
        source,
    })?;
    if source_bytes_after != SMOKE_SOURCE.as_bytes() {
        return Err(ProbeError::MutatedDuringProbe("fixed smoke source"));
    }
    let source_tree_after = collect_source_tree(
        &cubical_root,
        config.source_tree_pins().cubical().clone(),
        SourceTreeKind::Cubical,
    )?;
    if source_tree_after != source_tree_before {
        return Err(ProbeError::MutatedDuringProbe("Cubical source tree"));
    }
    let snapshot_source_tree_after = collect_source_tree(
        &cubical_snapshot_root,
        config.source_tree_pins().cubical().clone(),
        SourceTreeKind::Cubical,
    )?;
    if snapshot_source_tree_after != source_tree_before {
        return Err(ProbeError::MutatedDuringProbe(
            "Cubical source-tree snapshot",
        ));
    }
    let primitive_source_tree_after = collect_source_tree(
        &installed_primitive_root,
        config.source_tree_pins().agda_primitive().clone(),
        SourceTreeKind::AgdaPrimitive,
    )?;
    if primitive_source_tree_after != primitive_source_tree_before {
        return Err(ProbeError::MutatedDuringProbe(
            "Agda installed primitive source tree",
        ));
    }
    let snapshot_primitive_source_tree_after = collect_source_tree(
        &snapshot_primitive_root,
        config.source_tree_pins().agda_primitive().clone(),
        SourceTreeKind::AgdaPrimitive,
    )?;
    if snapshot_primitive_source_tree_after != primitive_source_tree_before {
        return Err(ProbeError::MutatedDuringProbe(
            "Agda primitive source-tree snapshot",
        ));
    }

    let agda_digest_after = digest_file(&agda_executable, "Agda executable")?;
    if agda_digest_after != agda_digest_before {
        return Err(ProbeError::MutatedDuringProbe("Agda executable"));
    }
    let git_digest_after = digest_file(&git_executable, "Git executable")?;
    if git_digest_after != git_digest_before {
        return Err(ProbeError::MutatedDuringProbe("Git executable"));
    }

    require_success("fixed Cubical Agda smoke checker", &checker_result)?;
    let timeout_millis = duration_millis(config.checker_timeout)?;
    let checker_evidence = CheckerInvocationEvidence::new(
        scratch_path_text,
        source_path_text,
        cubical_snapshot_root_text.clone(),
        snapshot_agda_data_directory_text.clone(),
        timeout_millis,
        checker_result.into_evidence(checker_arguments),
    );
    let agda_data_evidence = AgdaDataDirectoryEvidence::new(
        installed_agda_data_directory_text,
        snapshot_agda_data_directory_text,
        primitive_source_tree_before,
        data_directory_evidence,
    );
    let agda_evidence = AgdaToolchainEvidence::new(
        AGDA_EXPECTED_VERSION.to_owned(),
        agda_path_text,
        config.executable_pins.agda().clone(),
        agda_digest_before,
        agda_data_evidence,
        version_evidence,
    );
    let cubical_evidence = CubicalCheckoutEvidence::new(
        cubical_root_text,
        cubical_snapshot_root_text,
        CUBICAL_EXPECTED_COMMIT.to_owned(),
        git_path_text,
        config.executable_pins.git().clone(),
        git_digest_before,
        source_tree_before,
        CUBICAL_LIBRARY_NORMALIZED.len() as u64,
        EvidenceDigest::of_bytes(CUBICAL_LIBRARY_NORMALIZED.as_bytes()),
        top_evidence,
        head_before_evidence,
        head_after_evidence,
    );
    let evidence = BackendEvidenceManifest::new(agda_evidence, cubical_evidence, checker_evidence)?;
    scratch.cleanup()?;

    Ok(VerifiedPinnedBackend {
        evidence,
        _seal: private::CapabilitySeal,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SourceTreeKind {
    AgdaPrimitive,
    Cubical,
}

fn materialize_source_tree(
    source_root: &Path,
    snapshot_root: &Path,
    expected_digest: EvidenceDigest,
    kind: SourceTreeKind,
) -> Result<CubicalSourceTreeEvidence, ProbeError> {
    if let Some(parent) = snapshot_root.parent() {
        fs::create_dir_all(parent).map_err(|source| ProbeError::Io {
            context: format!(
                "create source snapshot parent directory {}",
                parent.display()
            ),
            source,
        })?;
    }
    create_new_directory(snapshot_root, "source snapshot root")?;
    collect_source_tree_inner(source_root, expected_digest, kind, Some(snapshot_root))
}

fn collect_source_tree(
    root: &Path,
    expected_digest: EvidenceDigest,
    kind: SourceTreeKind,
) -> Result<CubicalSourceTreeEvidence, ProbeError> {
    collect_source_tree_inner(root, expected_digest, kind, None)
}

fn collect_source_tree_inner(
    root: &Path,
    expected_digest: EvidenceDigest,
    kind: SourceTreeKind,
    snapshot_root: Option<&Path>,
) -> Result<CubicalSourceTreeEvidence, ProbeError> {
    let mut pending = vec![root.to_owned()];
    let mut visited_entries = 0_u64;
    let mut source_bytes = 0_u64;
    let mut files = Vec::new();

    while let Some(directory) = pending.pop() {
        let entries = fs::read_dir(&directory).map_err(|source| ProbeError::Io {
            context: format!("enumerate Cubical source directory {}", directory.display()),
            source,
        })?;
        for entry in entries {
            let entry = entry.map_err(|source| ProbeError::Io {
                context: format!("read Cubical directory entry in {}", directory.display()),
                source,
            })?;
            visited_entries = visited_entries
                .checked_add(1)
                .ok_or(ProbeError::SourceTreeLimit("entry count overflow"))?;
            if visited_entries > MAX_CUBICAL_TREE_ENTRIES {
                return Err(ProbeError::SourceTreeLimit(
                    "Cubical checkout contains too many filesystem entries",
                ));
            }

            let path = entry.path();
            if kind == SourceTreeKind::Cubical && directory == root && entry.file_name() == ".git" {
                continue;
            }
            let metadata = fs::symlink_metadata(&path).map_err(|source| ProbeError::Io {
                context: format!("inspect Cubical source entry {}", path.display()),
                source,
            })?;
            let file_type = metadata.file_type();
            if file_type.is_symlink() {
                return Err(ProbeError::UnsupportedSourceTreeEntry(path));
            }
            if file_type.is_dir() {
                pending.push(path);
                continue;
            }
            if !file_type.is_file() {
                return Err(ProbeError::UnsupportedSourceTreeEntry(path));
            }

            let relative = path
                .strip_prefix(root)
                .map_err(|_| ProbeError::PathEscapesRoot {
                    component: "Cubical source file",
                    root: root.to_owned(),
                    resolved: path.clone(),
                })?;
            let relative_text = relative
                .to_str()
                .ok_or_else(|| ProbeError::NonUtf8Path {
                    component: "Cubical source file",
                    path: path.clone(),
                })?
                .replace('\\', "/");
            if !is_source_file(kind, &relative_text) {
                continue;
            }
            if files.len() as u64 >= MAX_CUBICAL_SOURCE_FILES {
                return Err(ProbeError::SourceTreeLimit(
                    "Cubical checkout contains too many source files",
                ));
            }
            if metadata.len() > MAX_CUBICAL_SOURCE_FILE_BYTES {
                return Err(ProbeError::SourceTreeLimit(
                    "one Cubical source file exceeds the byte limit",
                ));
            }
            let raw_bytes = fs::read(&path).map_err(|source| ProbeError::Io {
                context: format!("read Cubical source file {}", path.display()),
                source,
            })?;
            if raw_bytes.len() as u64 > MAX_CUBICAL_SOURCE_FILE_BYTES {
                return Err(ProbeError::SourceTreeLimit(
                    "one Cubical source file grew beyond the byte limit",
                ));
            }
            let bytes = canonical_source_bytes(kind, &relative_text, &path, &raw_bytes)?;
            source_bytes = source_bytes
                .checked_add(bytes.len() as u64)
                .ok_or(ProbeError::SourceTreeLimit("source byte count overflow"))?;
            if source_bytes > MAX_CUBICAL_SOURCE_TREE_BYTES {
                return Err(ProbeError::SourceTreeLimit(
                    "source tree exceeds the byte limit",
                ));
            }
            if let Some(snapshot_root) = snapshot_root {
                let snapshot_path = snapshot_root.join(relative);
                if let Some(parent) = snapshot_path.parent() {
                    fs::create_dir_all(parent).map_err(|source| ProbeError::Io {
                        context: format!("create source snapshot directory {}", parent.display()),
                        source,
                    })?;
                }
                write_new_file(&snapshot_path, &bytes, "source snapshot file")?;
            }
            files.push(CubicalSourceFileEvidence::new(relative_text, &bytes));
        }
    }

    files.sort_by(|left, right| left.relative_path().cmp(right.relative_path()));
    let (contract_id, component) = match kind {
        SourceTreeKind::AgdaPrimitive => (
            AGDA_PRIMITIVE_SOURCE_TREE_CONTRACT_ID,
            "Agda primitive source-tree digest",
        ),
        SourceTreeKind::Cubical => (
            CUBICAL_SOURCE_TREE_CONTRACT_ID,
            "Cubical source-tree digest",
        ),
    };
    let observed_digest = source_tree_digest(contract_id, &files);
    require_digest_pin(component, &expected_digest, &observed_digest)?;
    match kind {
        SourceTreeKind::AgdaPrimitive => {
            AgdaPrimitiveSourceTreeEvidence::new_agda_primitive(expected_digest, files)
        }
        SourceTreeKind::Cubical => CubicalSourceTreeEvidence::new(expected_digest, files),
    }
    .map_err(ProbeError::Evidence)
}

fn is_source_file(kind: SourceTreeKind, relative_path: &str) -> bool {
    let agda_source = relative_path.ends_with(".agda")
        || relative_path.ends_with(".lagda")
        || relative_path.ends_with(".lagda.md")
        || relative_path.ends_with(".lagda.rst")
        || relative_path.ends_with(".lagda.tex");
    match kind {
        SourceTreeKind::AgdaPrimitive => {
            relative_path == "agda-builtins.agda-lib"
                || agda_source
                || relative_path.ends_with(".agdai")
        }
        SourceTreeKind::Cubical => relative_path == "cubical.agda-lib" || agda_source,
    }
}

fn canonical_source_bytes(
    kind: SourceTreeKind,
    relative_path: &str,
    path: &Path,
    raw_bytes: &[u8],
) -> Result<Vec<u8>, ProbeError> {
    if kind == SourceTreeKind::AgdaPrimitive && relative_path.ends_with(".agdai") {
        return Ok(raw_bytes.to_vec());
    }
    let text =
        std::str::from_utf8(raw_bytes).map_err(|_| ProbeError::NonUtf8Source(path.to_owned()))?;
    let normalized = text.replace("\r\n", "\n");
    if normalized.contains('\r') {
        return Err(ProbeError::BareCarriageReturn(path.to_owned()));
    }
    Ok(normalized.into_bytes())
}

fn require_digest_pin(
    component: &'static str,
    expected: &EvidenceDigest,
    actual: &EvidenceDigest,
) -> Result<(), ProbeError> {
    if expected == actual {
        Ok(())
    } else {
        Err(ProbeError::PinMismatch {
            component,
            expected: expected.to_string(),
            actual: actual.to_string(),
        })
    }
}

#[derive(Debug, Error)]
pub enum ProbeError {
    #[error("{component} path must be absolute: {path}")]
    RelativePath {
        component: &'static str,
        path: PathBuf,
    },
    #[error("{component} does not resolve to a regular file: {path}")]
    NotAFile {
        component: &'static str,
        path: PathBuf,
    },
    #[error("{component} does not resolve to a directory: {path}")]
    NotADirectory {
        component: &'static str,
        path: PathBuf,
    },
    #[error("{component} resolved outside {root}: {resolved}")]
    PathEscapesRoot {
        component: &'static str,
        root: PathBuf,
        resolved: PathBuf,
    },
    #[error("{component} path is not valid UTF-8 and cannot be bound into evidence: {path}")]
    NonUtf8Path {
        component: &'static str,
        path: PathBuf,
    },
    #[error("unsupported symlink or special entry in Cubical source tree: {0}")]
    UnsupportedSourceTreeEntry(PathBuf),
    #[error("Agda source is not valid UTF-8: {0}")]
    NonUtf8Source(PathBuf),
    #[error("Agda source contains a bare carriage return: {0}")]
    BareCarriageReturn(PathBuf),
    #[error("Cubical source-tree limit exceeded: {0}")]
    SourceTreeLimit(&'static str),
    #[error(
        "checker timeout must be positive and at most {maximum_millis} ms, found {actual_millis} ms"
    )]
    InvalidTimeout {
        maximum_millis: u64,
        actual_millis: u128,
    },
    #[error("{context}: {source}")]
    Io {
        context: String,
        #[source]
        source: io::Error,
    },
    #[error("could not allocate a private probe directory under {0}")]
    ScratchExhausted(PathBuf),
    #[error("failed to spawn {command}: {source}")]
    Spawn {
        command: String,
        #[source]
        source: io::Error,
    },
    #[error("failed to {operation} for {command}: {source}")]
    ProcessCleanup {
        command: String,
        operation: &'static str,
        #[source]
        source: io::Error,
    },
    #[error("{command} exceeded its {timeout_millis} ms timeout")]
    CommandTimedOut {
        command: String,
        timeout_millis: u64,
    },
    #[error("{command} terminated without a numeric exit code")]
    MissingExitCode { command: String },
    #[error("{command} failed with exit code {exit_code}; stdout={stdout:?}; stderr={stderr:?}")]
    CommandFailed {
        command: &'static str,
        exit_code: i32,
        stdout: String,
        stderr: String,
    },
    #[error("{stream} from {command} exceeded the {maximum_bytes}-byte capture limit")]
    OutputTooLarge {
        command: String,
        stream: &'static str,
        maximum_bytes: u64,
    },
    #[error("{stream} from {command} was not valid UTF-8")]
    NonUtf8Output {
        command: String,
        stream: &'static str,
    },
    #[error("{component} pin mismatch: expected {expected:?}, found {actual:?}")]
    PinMismatch {
        component: &'static str,
        expected: String,
        actual: String,
    },
    #[error("{0} changed while the readiness probe was running")]
    MutatedDuringProbe(&'static str),
    #[error("internal pinned backend contract error: {0}")]
    InternalContract(&'static str),
    #[error(transparent)]
    Evidence(#[from] BackendEvidenceError),
}

struct CompletedCommand {
    exit_code: i32,
    stdout: String,
    stderr: String,
}

impl CompletedCommand {
    fn into_evidence(self, arguments: Vec<String>) -> ProcessEvidence {
        ProcessEvidence::new(
            arguments,
            self.exit_code,
            CapturedOutput::new(self.stdout),
            CapturedOutput::new(self.stderr),
        )
    }
}

struct ScratchDirectory {
    path: PathBuf,
    cleaned: bool,
}

impl ScratchDirectory {
    fn create(root: &Path) -> Result<Self, ProbeError> {
        let process_id = std::process::id();
        for _ in 0..MAX_SCRATCH_ATTEMPTS {
            let counter = SCRATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = root.join(format!("pen-gf2-agda-probe-{process_id}-{counter}"));
            match fs::create_dir(&path) {
                Ok(()) => {
                    return Ok(Self {
                        path,
                        cleaned: false,
                    });
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(source) => {
                    return Err(ProbeError::Io {
                        context: format!("create private probe directory {}", path.display()),
                        source,
                    });
                }
            }
        }
        Err(ProbeError::ScratchExhausted(root.to_owned()))
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn cleanup(mut self) -> Result<(), ProbeError> {
        fs::remove_dir_all(&self.path).map_err(|source| ProbeError::Io {
            context: format!("remove private probe directory {}", self.path.display()),
            source,
        })?;
        self.cleaned = true;
        Ok(())
    }
}

impl Drop for ScratchDirectory {
    fn drop(&mut self) {
        if !self.cleaned {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

fn validate_timeout(timeout: Duration) -> Result<(), ProbeError> {
    if timeout.is_zero() || timeout > MAX_CHECKER_TIMEOUT {
        return Err(ProbeError::InvalidTimeout {
            maximum_millis: MAX_CHECKER_TIMEOUT_MILLIS,
            actual_millis: timeout.as_millis(),
        });
    }
    Ok(())
}

fn duration_millis(duration: Duration) -> Result<u64, ProbeError> {
    u64::try_from(duration.as_millis()).map_err(|_| ProbeError::InvalidTimeout {
        maximum_millis: MAX_CHECKER_TIMEOUT_MILLIS,
        actual_millis: duration.as_millis(),
    })
}

fn require_absolute(path: &Path, component: &'static str) -> Result<(), ProbeError> {
    if path.is_absolute() {
        Ok(())
    } else {
        Err(ProbeError::RelativePath {
            component,
            path: path.to_owned(),
        })
    }
}

fn canonical_file(path: &Path, component: &'static str) -> Result<PathBuf, ProbeError> {
    let canonical = fs::canonicalize(path).map_err(|source| ProbeError::Io {
        context: format!("resolve {component} {}", path.display()),
        source,
    })?;
    if canonical.is_file() {
        Ok(canonical)
    } else {
        Err(ProbeError::NotAFile {
            component,
            path: canonical,
        })
    }
}

fn canonical_directory(path: &Path, component: &'static str) -> Result<PathBuf, ProbeError> {
    let canonical = fs::canonicalize(path).map_err(|source| ProbeError::Io {
        context: format!("resolve {component} {}", path.display()),
        source,
    })?;
    if canonical.is_dir() {
        Ok(canonical)
    } else {
        Err(ProbeError::NotADirectory {
            component,
            path: canonical,
        })
    }
}

fn canonical_reported_directory(
    stdout: &str,
    component: &'static str,
) -> Result<PathBuf, ProbeError> {
    let reported = stdout.trim_end_matches(['\r', '\n']);
    if reported.is_empty() || reported.contains(['\r', '\n']) {
        return Err(ProbeError::PinMismatch {
            component,
            expected: "one absolute directory path followed by optional line ending".to_owned(),
            actual: stdout.to_owned(),
        });
    }
    let path = Path::new(reported);
    require_absolute(path, component)?;
    canonical_directory(path, component)
}

fn path_text(path: &Path, component: &'static str) -> Result<String, ProbeError> {
    path.to_str()
        .map(str::to_owned)
        .ok_or_else(|| ProbeError::NonUtf8Path {
            component,
            path: path.to_owned(),
        })
}

fn digest_file(path: &Path, component: &'static str) -> Result<EvidenceDigest, ProbeError> {
    let mut file = File::open(path).map_err(|source| ProbeError::Io {
        context: format!("open {component} {}", path.display()),
        source,
    })?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).map_err(|source| ProbeError::Io {
            context: format!("hash {component} {}", path.display()),
            source,
        })?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    EvidenceDigest::parse(format!("blake3:{}", hasher.finalize().to_hex()))
        .map_err(|_| ProbeError::InternalContract("BLAKE3 emitted a noncanonical digest"))
}

fn create_new_directory(path: &Path, component: &'static str) -> Result<(), ProbeError> {
    fs::create_dir(path).map_err(|source| ProbeError::Io {
        context: format!("create {component} {}", path.display()),
        source,
    })
}

fn write_new_file(path: &Path, bytes: &[u8], component: &'static str) -> Result<(), ProbeError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|source| ProbeError::Io {
            context: format!("create {component} {}", path.display()),
            source,
        })?;
    file.write_all(bytes).map_err(|source| ProbeError::Io {
        context: format!("write {component} {}", path.display()),
        source,
    })?;
    file.sync_all().map_err(|source| ProbeError::Io {
        context: format!("sync {component} {}", path.display()),
        source,
    })
}

fn run_captured(
    executable: &Path,
    arguments: &[String],
    working_directory: &Path,
    timeout: Duration,
    log_stem: &str,
) -> Result<CompletedCommand, ProbeError> {
    run_captured_inner(
        executable,
        arguments,
        working_directory,
        timeout,
        log_stem,
        None,
    )
}

fn run_captured_with_agda_data_directory(
    executable: &Path,
    arguments: &[String],
    working_directory: &Path,
    timeout: Duration,
    log_stem: &str,
    agda_data_directory: &Path,
) -> Result<CompletedCommand, ProbeError> {
    run_captured_inner(
        executable,
        arguments,
        working_directory,
        timeout,
        log_stem,
        Some(agda_data_directory),
    )
}

fn run_captured_inner(
    executable: &Path,
    arguments: &[String],
    working_directory: &Path,
    timeout: Duration,
    log_stem: &str,
    agda_data_directory: Option<&Path>,
) -> Result<CompletedCommand, ProbeError> {
    let command_label = format!("{} {}", executable.display(), arguments.join(" "));
    let stdout_path = working_directory.join(format!(".{log_stem}.stdout"));
    let stderr_path = working_directory.join(format!(".{log_stem}.stderr"));
    let stdout_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&stdout_path)
        .map_err(|source| ProbeError::Io {
            context: format!("create stdout capture {}", stdout_path.display()),
            source,
        })?;
    let stderr_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&stderr_path)
        .map_err(|source| ProbeError::Io {
            context: format!("create stderr capture {}", stderr_path.display()),
            source,
        })?;

    let mut command = Command::new(executable);
    command
        .args(arguments)
        .current_dir(working_directory)
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file));
    configure_sanitized_environment(&mut command, working_directory, agda_data_directory);
    let mut child = command.group_spawn().map_err(|source| ProbeError::Spawn {
        command: command_label.clone(),
        source,
    })?;

    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                terminate_process_group(&mut child, &command_label)?;
                break status;
            }
            Ok(None) => {}
            Err(source) => {
                terminate_process_group(&mut child, &command_label)?;
                return Err(ProbeError::Io {
                    context: format!("poll {command_label}"),
                    source,
                });
            }
        }
        match capture_exceeds_limit(&stdout_path) {
            Ok(true) => {
                terminate_process_group(&mut child, &command_label)?;
                return Err(ProbeError::OutputTooLarge {
                    command: command_label,
                    stream: "stdout",
                    maximum_bytes: MAX_CAPTURE_BYTES,
                });
            }
            Ok(false) => {}
            Err(error) => {
                terminate_process_group(&mut child, &command_label)?;
                return Err(error);
            }
        }
        match capture_exceeds_limit(&stderr_path) {
            Ok(true) => {
                terminate_process_group(&mut child, &command_label)?;
                return Err(ProbeError::OutputTooLarge {
                    command: command_label,
                    stream: "stderr",
                    maximum_bytes: MAX_CAPTURE_BYTES,
                });
            }
            Ok(false) => {}
            Err(error) => {
                terminate_process_group(&mut child, &command_label)?;
                return Err(error);
            }
        }
        let elapsed = started.elapsed();
        if elapsed >= timeout {
            terminate_process_group(&mut child, &command_label)?;
            return Err(ProbeError::CommandTimedOut {
                command: command_label,
                timeout_millis: duration_millis(timeout)?,
            });
        }
        thread::sleep(PROCESS_POLL_INTERVAL.min(timeout - elapsed));
    };
    let exit_code = status.code().ok_or_else(|| ProbeError::MissingExitCode {
        command: command_label.clone(),
    })?;
    let stdout = read_bounded_utf8(&stdout_path, &command_label, "stdout")?;
    let stderr = read_bounded_utf8(&stderr_path, &command_label, "stderr")?;
    Ok(CompletedCommand {
        exit_code,
        stdout,
        stderr,
    })
}

fn configure_sanitized_environment(
    command: &mut Command,
    working_directory: &Path,
    agda_data_directory: Option<&Path>,
) {
    command.env_clear();
    for key in ["SystemRoot", "WINDIR", "ComSpec", "PATHEXT"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    command
        .env("HOME", working_directory)
        .env("USERPROFILE", working_directory)
        .env("XDG_CONFIG_HOME", working_directory)
        .env("APPDATA", working_directory)
        .env("LOCALAPPDATA", working_directory)
        .env("TMP", working_directory)
        .env("TEMP", working_directory)
        .env("TMPDIR", working_directory)
        .env("AGDA_DIR", working_directory.join(".agda-app"))
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_COUNT", "0")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("LC_ALL", "C.UTF-8")
        .env("LANG", "C.UTF-8");
    if let Some(data_directory) = agda_data_directory {
        command.env("Agda_datadir", data_directory);
    }
}

fn terminate_process_group(
    child: &mut command_group::GroupChild,
    command: &str,
) -> Result<(), ProbeError> {
    match child.kill() {
        Ok(()) => {}
        Err(error) if process_group_is_absent(&error) => {}
        Err(source) => {
            return Err(ProbeError::ProcessCleanup {
                command: command.to_owned(),
                operation: "terminate process group",
                source,
            });
        }
    }
    child
        .wait()
        .map(|_| ())
        .map_err(|source| ProbeError::ProcessCleanup {
            command: command.to_owned(),
            operation: "wait for process group termination",
            source,
        })
}

fn process_group_is_absent(error: &io::Error) -> bool {
    if matches!(
        error.kind(),
        io::ErrorKind::InvalidInput | io::ErrorKind::NotFound
    ) {
        return true;
    }
    // Rust releases have mapped POSIX ESRCH to different ErrorKind variants.
    // The raw value is stable across the Unix targets supported by this crate.
    #[cfg(unix)]
    if error.raw_os_error() == Some(3) {
        return true;
    }
    false
}

fn capture_exceeds_limit(path: &Path) -> Result<bool, ProbeError> {
    fs::metadata(path)
        .map(|metadata| metadata.len() > MAX_CAPTURE_BYTES)
        .map_err(|source| ProbeError::Io {
            context: format!("inspect process capture {}", path.display()),
            source,
        })
}

fn read_bounded_utf8(
    path: &Path,
    command: &str,
    stream: &'static str,
) -> Result<String, ProbeError> {
    let metadata = fs::metadata(path).map_err(|source| ProbeError::Io {
        context: format!("inspect {stream} capture {}", path.display()),
        source,
    })?;
    if metadata.len() > MAX_CAPTURE_BYTES {
        return Err(ProbeError::OutputTooLarge {
            command: command.to_owned(),
            stream,
            maximum_bytes: MAX_CAPTURE_BYTES,
        });
    }
    let bytes = fs::read(path).map_err(|source| ProbeError::Io {
        context: format!("read {stream} capture {}", path.display()),
        source,
    })?;
    if bytes.len() as u64 > MAX_CAPTURE_BYTES {
        return Err(ProbeError::OutputTooLarge {
            command: command.to_owned(),
            stream,
            maximum_bytes: MAX_CAPTURE_BYTES,
        });
    }
    String::from_utf8(bytes).map_err(|_| ProbeError::NonUtf8Output {
        command: command.to_owned(),
        stream,
    })
}

fn require_success(command: &'static str, completed: &CompletedCommand) -> Result<(), ProbeError> {
    if completed.exit_code == 0 {
        Ok(())
    } else {
        Err(ProbeError::CommandFailed {
            command,
            exit_code: completed.exit_code,
            stdout: completed.stdout.clone(),
            stderr: completed.stderr.clone(),
        })
    }
}

fn verify_git_top_level(stdout: &str, expected_root: &Path) -> Result<(), ProbeError> {
    let reported = stdout.trim();
    let reported_path = PathBuf::from(reported);
    if !reported_path.is_absolute() {
        return Err(ProbeError::PinMismatch {
            component: "Git top-level path",
            expected: expected_root.display().to_string(),
            actual: reported.to_owned(),
        });
    }
    let canonical = fs::canonicalize(&reported_path).map_err(|source| ProbeError::Io {
        context: format!("resolve Git-reported top level {}", reported_path.display()),
        source,
    })?;
    if canonical == expected_root {
        Ok(())
    } else {
        Err(ProbeError::PinMismatch {
            component: "Git top-level path",
            expected: expected_root.display().to_string(),
            actual: canonical.display().to_string(),
        })
    }
}

fn verify_head(stdout: &str, when: &'static str) -> Result<(), ProbeError> {
    let actual = stdout.trim();
    if actual == CUBICAL_EXPECTED_COMMIT {
        Ok(())
    } else {
        Err(ProbeError::PinMismatch {
            component: when,
            expected: CUBICAL_EXPECTED_COMMIT.to_owned(),
            actual: actual.to_owned(),
        })
    }
}

fn validate_library_file(bytes: &[u8]) -> Result<(), ProbeError> {
    let text = std::str::from_utf8(bytes).map_err(|_| ProbeError::PinMismatch {
        component: "Cubical library file encoding",
        expected: "UTF-8".to_owned(),
        actual: "non-UTF-8 bytes".to_owned(),
    })?;
    let normalized = text.replace("\r\n", "\n");
    if normalized.contains('\r') {
        return Err(ProbeError::PinMismatch {
            component: "Cubical library file line endings",
            expected: "LF or CRLF".to_owned(),
            actual: "bare carriage return".to_owned(),
        });
    }
    if normalized == CUBICAL_LIBRARY_NORMALIZED {
        Ok(())
    } else {
        Err(ProbeError::PinMismatch {
            component: "Cubical library declaration",
            expected: CUBICAL_LIBRARY_NORMALIZED.to_owned(),
            actual: normalized,
        })
    }
}

fn extend_arguments(prefix: &[String], suffix: &[&str]) -> Vec<String> {
    prefix
        .iter()
        .cloned()
        .chain(suffix.iter().map(|argument| (*argument).to_owned()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        CUBICAL_EXPECTED_SOURCE_TREE_DIGEST, CUBICAL_LIBRARY_NORMALIZED, ExecutableDigestPins,
        MAX_CAPTURE_BYTES, MAX_CHECKER_TIMEOUT, PinnedBackendConfig, ProbeError, SCRATCH_COUNTER,
        SourceTreeDigestPins, SourceTreeKind, canonical_source_bytes, capture_exceeds_limit,
        probe_pinned_backend, process_group_is_absent, validate_library_file,
    };
    use crate::{AGDA_REFERENCE_PRIMITIVE_TREE_DIGEST, EvidenceDigest};
    use std::fs;
    use std::io;
    use std::path::Path;
    use std::sync::atomic::Ordering;
    use std::time::Duration;

    fn dummy_pins() -> ExecutableDigestPins {
        ExecutableDigestPins::new(
            EvidenceDigest::of_bytes(b"agda-pin"),
            EvidenceDigest::of_bytes(b"git-pin"),
        )
    }

    fn reviewed_tree_pin() -> EvidenceDigest {
        EvidenceDigest::parse(CUBICAL_EXPECTED_SOURCE_TREE_DIGEST)
            .expect("reviewed Cubical source-tree pin")
    }

    fn reviewed_primitive_tree_pin() -> EvidenceDigest {
        EvidenceDigest::parse(AGDA_REFERENCE_PRIMITIVE_TREE_DIGEST)
            .expect("reviewed Agda primitive source-tree pin")
    }

    fn reviewed_source_tree_pins() -> SourceTreeDigestPins {
        SourceTreeDigestPins::new(reviewed_primitive_tree_pin(), reviewed_tree_pin())
    }

    #[test]
    fn exact_library_contract_accepts_lf_and_crlf_only() {
        assert!(validate_library_file(CUBICAL_LIBRARY_NORMALIZED.as_bytes()).is_ok());
        let crlf = CUBICAL_LIBRARY_NORMALIZED.replace('\n', "\r\n");
        assert!(validate_library_file(crlf.as_bytes()).is_ok());

        let extra_flag = CUBICAL_LIBRARY_NORMALIZED
            .replace("--guardedness", "--guardedness --allow-unsolved-metas");
        assert!(validate_library_file(extra_flag.as_bytes()).is_err());
        let changed_name = CUBICAL_LIBRARY_NORMALIZED.replace("cubical-0.9", "cubical");
        assert!(validate_library_file(changed_name.as_bytes()).is_err());
        let changed_include = CUBICAL_LIBRARY_NORMALIZED.replace("include: .", "include: Cubical");
        assert!(validate_library_file(changed_include.as_bytes()).is_err());
    }

    #[test]
    fn source_manifest_and_snapshot_canonicalize_text_line_endings() {
        let path = Path::new("Cubical/Example.agda");
        let lf = b"module Cubical.Example where\nvalue = 1\n";
        let crlf = b"module Cubical.Example where\r\nvalue = 1\r\n";
        assert_eq!(
            canonical_source_bytes(SourceTreeKind::Cubical, "Cubical/Example.agda", path, lf)
                .expect("LF source"),
            canonical_source_bytes(SourceTreeKind::Cubical, "Cubical/Example.agda", path, crlf)
                .expect("CRLF source")
        );
        assert!(matches!(
            canonical_source_bytes(
                SourceTreeKind::Cubical,
                "Cubical/Example.agda",
                path,
                b"module Cubical.Example where\rvalue = 1\n"
            ),
            Err(ProbeError::BareCarriageReturn(_))
        ));

        let interface = b"binary\r\ninterface";
        assert_eq!(
            canonical_source_bytes(
                SourceTreeKind::AgdaPrimitive,
                "_build/2.8.0/agda/Agda/Primitive.agdai",
                path,
                interface
            )
            .expect("primitive interface bytes"),
            interface
        );
    }

    #[test]
    fn process_group_absence_classification_is_portable() {
        assert!(process_group_is_absent(&io::Error::from(
            io::ErrorKind::InvalidInput
        )));
        assert!(process_group_is_absent(&io::Error::from(
            io::ErrorKind::NotFound
        )));
        assert!(!process_group_is_absent(&io::Error::from(
            io::ErrorKind::PermissionDenied
        )));

        #[cfg(unix)]
        assert!(process_group_is_absent(&io::Error::from_raw_os_error(3)));
    }

    #[test]
    fn timeout_contract_fails_closed_before_touching_paths() {
        for timeout in [
            Duration::ZERO,
            MAX_CHECKER_TIMEOUT + Duration::from_millis(1),
        ] {
            let config = PinnedBackendConfig::new(
                "relative-agda",
                "relative-git",
                dummy_pins(),
                reviewed_source_tree_pins(),
                "relative-cubical",
                "relative-scratch",
                timeout,
            );
            assert!(matches!(
                probe_pinned_backend(&config),
                Err(ProbeError::InvalidTimeout { .. })
            ));
        }
    }

    #[test]
    fn relative_tool_paths_are_rejected() {
        let config = PinnedBackendConfig::new(
            "agda",
            "git",
            dummy_pins(),
            reviewed_source_tree_pins(),
            "cubical",
            "scratch",
            Duration::from_secs(1),
        );
        assert!(matches!(
            probe_pinned_backend(&config),
            Err(ProbeError::RelativePath {
                component: "Agda executable",
                ..
            })
        ));
    }

    #[test]
    fn executable_digest_mismatch_fails_before_invocation() {
        let counter = SCRATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pen-gf2-agda-pin-mismatch-test-{}-{counter}",
            std::process::id()
        ));
        let cubical = root.join("cubical");
        let scratch = root.join("scratch");
        fs::create_dir_all(&cubical).expect("create cubical fixture");
        fs::create_dir(&scratch).expect("create scratch fixture");
        let agda = root.join("agda");
        let git = root.join("git");
        fs::write(&agda, b"not invoked").expect("write Agda fixture");
        fs::write(&git, b"not invoked").expect("write Git fixture");

        let config = PinnedBackendConfig::new(
            &agda,
            &git,
            dummy_pins(),
            reviewed_source_tree_pins(),
            &cubical,
            &scratch,
            Duration::from_secs(1),
        );
        assert!(matches!(
            probe_pinned_backend(&config),
            Err(ProbeError::PinMismatch {
                component: "Agda executable digest",
                ..
            })
        ));
        fs::remove_dir_all(root).expect("remove pin fixture");
    }

    #[test]
    fn oversized_capture_is_detected_from_file_metadata() {
        let counter = SCRATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "pen-gf2-agda-capture-limit-test-{}-{counter}",
            std::process::id()
        ));
        let file = fs::File::create(&path).expect("create sparse test file");
        file.set_len(MAX_CAPTURE_BYTES + 1)
            .expect("extend sparse test file");
        drop(file);
        assert!(matches!(capture_exceeds_limit(&path), Ok(true)));
        fs::remove_file(path).expect("remove sparse test file");
    }

    #[test]
    #[ignore = "requires PEN_GF2_AGDA_EXECUTABLE, PEN_GF2_GIT_EXECUTABLE, and PEN_GF2_CUBICAL_ROOT"]
    fn installed_pinned_backend_passes_fixed_smoke_probe() {
        let agda = std::env::var_os("PEN_GF2_AGDA_EXECUTABLE")
            .expect("set PEN_GF2_AGDA_EXECUTABLE to an absolute Agda 2.8.0 executable");
        let git = std::env::var_os("PEN_GF2_GIT_EXECUTABLE")
            .expect("set PEN_GF2_GIT_EXECUTABLE to an absolute Git executable");
        let cubical = std::env::var_os("PEN_GF2_CUBICAL_ROOT")
            .expect("set PEN_GF2_CUBICAL_ROOT to the reviewed cubical-0.9 source tree");
        let pins = ExecutableDigestPins::new(
            EvidenceDigest::of_bytes(&fs::read(&agda).expect("read Agda executable")),
            EvidenceDigest::of_bytes(&fs::read(&git).expect("read Git executable")),
        );
        let config = PinnedBackendConfig::new(
            agda,
            git,
            pins,
            reviewed_source_tree_pins(),
            cubical,
            std::env::temp_dir(),
            Duration::from_secs(120),
        );
        let verified = probe_pinned_backend(&config).expect("pinned backend probe");
        verified.evidence().validate().expect("evidence integrity");
        let wire = serde_json::to_string(verified.evidence()).expect("serialize evidence");
        let replayed: crate::BackendEvidenceManifest =
            serde_json::from_str(&wire).expect("strict evidence replay");
        assert_eq!(replayed, *verified.evidence());
    }
}
