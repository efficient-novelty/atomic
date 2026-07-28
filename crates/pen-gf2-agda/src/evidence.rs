use crate::digest::EvidenceDigest;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize};
use std::path::Path;
use thiserror::Error;

pub const EVIDENCE_SCHEMA_VERSION: u16 = 3;
pub const BACKEND_CONTRACT_ID: &str = "pen-gf2-agda/pinned-readiness/v3";
pub const MAX_CHECKER_TIMEOUT_MILLIS: u64 = 300_000;
pub const AGDA_EXPECTED_VERSION: &str = "2.8.0";
/// Reference digest for the local Agda 2.8.0 primitive-runtime installation
/// used by the ignored integration test.
///
/// Unlike the repository-pinned Cubical source digest, a production caller
/// must supply an independently reviewed primitive-runtime digest for the
/// exact Agda distribution paired with its trusted executable.
pub const AGDA_REFERENCE_PRIMITIVE_TREE_DIGEST: &str =
    "blake3:2cae2bc83c34013b7cf4221bc99509bfe74a15404be43dd04367a7733d85f52d";
pub const CUBICAL_EXPECTED_COMMIT: &str = "b150186d2544e7efeddd31e5d14a8b9ecbb100f7";
pub const CUBICAL_EXPECTED_LIBRARY_NAME: &str = "cubical-0.9";
pub const CUBICAL_EXPECTED_INCLUDE: &str = ".";
pub const CUBICAL_EXPECTED_SOURCE_TREE_DIGEST: &str =
    "blake3:51035e9da96b06d544aa23051cb22607b6a983a134170623b599fcba4837cb5d";
pub const CUBICAL_EXPECTED_FLAGS: &[&str] = &[
    "--safe",
    "--cubical",
    "--no-import-sorts",
    "-WnoUnsupportedIndexedMatch",
    "--guardedness",
];
pub(crate) const AGDA_DATA_SNAPSHOT_DIRECTORY_NAME: &str = "agda-data";
pub(crate) const AGDA_PRIMITIVE_RELATIVE_ROOT: &str = "lib/prim";
pub(crate) const AGDA_PRIMITIVE_SOURCE_TREE_CONTRACT_ID: &str =
    "pen-gf2-agda/agda-primitive-source-tree/v1";
pub(crate) const CUBICAL_SNAPSHOT_DIRECTORY_NAME: &str = "cubical-source";
pub(crate) const CUBICAL_SOURCE_TREE_CONTRACT_ID: &str = "pen-gf2-agda/cubical-source-tree/v1";
pub const SMOKE_MODULE_NAME: &str = "PenGF2BackendSmoke";
pub const SMOKE_FILE_NAME: &str = "PenGF2BackendSmoke.agda";

// Snapshot updated only when the versioned, internally fixed source contract
// changes. The probe independently hashes the bytes it actually writes.
pub const SMOKE_SOURCE_DIGEST: &str =
    "blake3:0a5b5f57352b0bdf293efb0c66ba4c159677a515ad1bc63a4cab8c873d3f4829";

pub(crate) const SMOKE_SOURCE: &str = concat!(
    "{-# OPTIONS --safe --cubical #-}\n",
    "module PenGF2BackendSmoke where\n",
    "\n",
    "open import Cubical.Foundations.Prelude using (Level; Type; _≡_; refl)\n",
    "\n",
    "identityPath : {ℓ : Level} {A : Type ℓ} (x : A) → x ≡ x\n",
    "identityPath x = refl\n",
);

pub(crate) const CUBICAL_LIBRARY_NORMALIZED: &str = concat!(
    "name: cubical-0.9\n",
    "include: .\n",
    "depend:\n",
    "flags: --safe --cubical --no-import-sorts ",
    "-WnoUnsupportedIndexedMatch --guardedness\n",
);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapturedOutput {
    byte_length: u64,
    digest: EvidenceDigest,
    text: String,
}

impl CapturedOutput {
    pub(crate) fn new(text: String) -> Self {
        Self {
            byte_length: text.len() as u64,
            digest: EvidenceDigest::of_bytes(text.as_bytes()),
            text,
        }
    }

    pub fn byte_length(&self) -> u64 {
        self.byte_length
    }

    pub fn digest(&self) -> &EvidenceDigest {
        &self.digest
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    fn validate(&self) -> Result<(), BackendEvidenceError> {
        if self.byte_length != self.text.len() as u64 {
            return Err(BackendEvidenceError::Invariant(
                "captured output byte length does not match its text",
            ));
        }
        if self.digest != EvidenceDigest::of_bytes(self.text.as_bytes()) {
            return Err(BackendEvidenceError::Invariant(
                "captured output digest does not match its text",
            ));
        }
        Ok(())
    }

    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u64(self.byte_length);
        encoder.text(self.digest.as_str());
        encoder.text(&self.text);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessEvidence {
    arguments: Vec<String>,
    exit_code: i32,
    stdout: CapturedOutput,
    stderr: CapturedOutput,
}

impl ProcessEvidence {
    pub(crate) fn new(
        arguments: Vec<String>,
        exit_code: i32,
        stdout: CapturedOutput,
        stderr: CapturedOutput,
    ) -> Self {
        Self {
            arguments,
            exit_code,
            stdout,
            stderr,
        }
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }

    pub fn stdout(&self) -> &CapturedOutput {
        &self.stdout
    }

    pub fn stderr(&self) -> &CapturedOutput {
        &self.stderr
    }

    fn validate(&self) -> Result<(), BackendEvidenceError> {
        self.stdout.validate()?;
        self.stderr.validate()
    }

    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.text_sequence(&self.arguments);
        encoder.i32(self.exit_code);
        self.stdout.encode_canonical(encoder);
        self.stderr.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixedSmokeSourceEvidence {
    module_name: String,
    file_name: String,
    byte_length: u64,
    digest: EvidenceDigest,
}

impl FixedSmokeSourceEvidence {
    pub(crate) fn current() -> Self {
        Self {
            module_name: SMOKE_MODULE_NAME.to_owned(),
            file_name: SMOKE_FILE_NAME.to_owned(),
            byte_length: SMOKE_SOURCE.len() as u64,
            digest: EvidenceDigest::of_bytes(SMOKE_SOURCE.as_bytes()),
        }
    }

    pub fn module_name(&self) -> &str {
        &self.module_name
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn byte_length(&self) -> u64 {
        self.byte_length
    }

    pub fn digest(&self) -> &EvidenceDigest {
        &self.digest
    }

    fn validate(&self) -> Result<(), BackendEvidenceError> {
        let current = Self::current();
        if self != &current {
            return Err(BackendEvidenceError::Invariant(
                "smoke source evidence does not match the fixed source contract",
            ));
        }
        if self.digest.as_str() != SMOKE_SOURCE_DIGEST {
            return Err(BackendEvidenceError::Invariant(
                "fixed smoke source digest snapshot is inconsistent",
            ));
        }
        Ok(())
    }

    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.text(&self.module_name);
        encoder.text(&self.file_name);
        encoder.u64(self.byte_length);
        encoder.text(self.digest.as_str());
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgdaDataDirectoryEvidence {
    installed_data_directory_path: String,
    snapshot_data_directory_path: String,
    primitive_source_tree: AgdaPrimitiveSourceTreeEvidence,
    data_directory_probe: ProcessEvidence,
}

impl AgdaDataDirectoryEvidence {
    pub(crate) fn new(
        installed_data_directory_path: String,
        snapshot_data_directory_path: String,
        primitive_source_tree: AgdaPrimitiveSourceTreeEvidence,
        data_directory_probe: ProcessEvidence,
    ) -> Self {
        Self {
            installed_data_directory_path,
            snapshot_data_directory_path,
            primitive_source_tree,
            data_directory_probe,
        }
    }

    pub fn installed_data_directory_path(&self) -> &str {
        &self.installed_data_directory_path
    }

    pub fn snapshot_data_directory_path(&self) -> &str {
        &self.snapshot_data_directory_path
    }

    pub fn primitive_source_tree(&self) -> &AgdaPrimitiveSourceTreeEvidence {
        &self.primitive_source_tree
    }

    pub fn data_directory_probe(&self) -> &ProcessEvidence {
        &self.data_directory_probe
    }

    fn validate(&self) -> Result<(), BackendEvidenceError> {
        require_absolute_path(
            &self.installed_data_directory_path,
            "Agda installed data-directory path is not absolute",
        )?;
        require_absolute_path(
            &self.snapshot_data_directory_path,
            "Agda snapshot data-directory path is not absolute",
        )?;
        if self.primitive_source_tree.contract_id() != AGDA_PRIMITIVE_SOURCE_TREE_CONTRACT_ID {
            return Err(BackendEvidenceError::Invariant(
                "Agda primitive source tree uses the wrong evidence contract",
            ));
        }
        self.primitive_source_tree.validate()?;
        self.data_directory_probe.validate()?;
        if self.data_directory_probe.arguments != ["--print-agda-data-dir"]
            || self.data_directory_probe.exit_code != 0
        {
            return Err(BackendEvidenceError::Invariant(
                "Agda data-directory probe command is not canonical and successful",
            ));
        }
        if !equivalent_absolute_path_text(
            self.data_directory_probe.stdout.text().trim(),
            &self.installed_data_directory_path,
        ) {
            return Err(BackendEvidenceError::Invariant(
                "Agda installed data directory disagrees with captured probe output",
            ));
        }
        Ok(())
    }

    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.text(&self.installed_data_directory_path);
        encoder.text(&self.snapshot_data_directory_path);
        self.primitive_source_tree.encode_canonical(encoder);
        self.data_directory_probe.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgdaToolchainEvidence {
    expected_version: String,
    observed_version: String,
    executable_path: String,
    expected_executable_digest: EvidenceDigest,
    observed_executable_digest: EvidenceDigest,
    data_directory: AgdaDataDirectoryEvidence,
    version_probe: ProcessEvidence,
}

impl AgdaToolchainEvidence {
    pub(crate) fn new(
        observed_version: String,
        executable_path: String,
        expected_executable_digest: EvidenceDigest,
        observed_executable_digest: EvidenceDigest,
        data_directory: AgdaDataDirectoryEvidence,
        version_probe: ProcessEvidence,
    ) -> Self {
        Self {
            expected_version: AGDA_EXPECTED_VERSION.to_owned(),
            observed_version,
            executable_path,
            expected_executable_digest,
            observed_executable_digest,
            data_directory,
            version_probe,
        }
    }

    pub fn expected_version(&self) -> &str {
        &self.expected_version
    }

    pub fn observed_version(&self) -> &str {
        &self.observed_version
    }

    pub fn executable_path(&self) -> &str {
        &self.executable_path
    }

    pub fn expected_executable_digest(&self) -> &EvidenceDigest {
        &self.expected_executable_digest
    }

    pub fn observed_executable_digest(&self) -> &EvidenceDigest {
        &self.observed_executable_digest
    }

    pub fn data_directory(&self) -> &AgdaDataDirectoryEvidence {
        &self.data_directory
    }

    pub fn version_probe(&self) -> &ProcessEvidence {
        &self.version_probe
    }

    fn validate(&self) -> Result<(), BackendEvidenceError> {
        require_absolute_path(
            &self.executable_path,
            "Agda executable path is not absolute",
        )?;
        if self.expected_version != AGDA_EXPECTED_VERSION
            || self.observed_version != AGDA_EXPECTED_VERSION
        {
            return Err(BackendEvidenceError::Invariant(
                "Agda evidence does not match the exact version pin",
            ));
        }
        if self.expected_executable_digest != self.observed_executable_digest {
            return Err(BackendEvidenceError::Invariant(
                "Agda executable bytes do not match the trusted digest pin",
            ));
        }
        self.data_directory.validate()?;
        self.version_probe.validate()?;
        if self.version_probe.arguments != ["--version"] || self.version_probe.exit_code != 0 {
            return Err(BackendEvidenceError::Invariant(
                "Agda version probe command is not canonical and successful",
            ));
        }
        let parsed = parse_agda_version(self.version_probe.stdout.text()).ok_or(
            BackendEvidenceError::Invariant("Agda version output has an unknown format"),
        )?;
        if parsed != self.observed_version {
            return Err(BackendEvidenceError::Invariant(
                "Agda observed version disagrees with captured version output",
            ));
        }
        Ok(())
    }

    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.text(&self.expected_version);
        encoder.text(&self.observed_version);
        encoder.text(&self.executable_path);
        encoder.text(self.expected_executable_digest.as_str());
        encoder.text(self.observed_executable_digest.as_str());
        self.data_directory.encode_canonical(encoder);
        self.version_probe.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CubicalSourceFileEvidence {
    relative_path: String,
    byte_length: u64,
    digest: EvidenceDigest,
}

impl CubicalSourceFileEvidence {
    pub(crate) fn new(relative_path: String, bytes: &[u8]) -> Self {
        Self {
            relative_path,
            byte_length: bytes.len() as u64,
            digest: EvidenceDigest::of_bytes(bytes),
        }
    }

    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }

    pub fn byte_length(&self) -> u64 {
        self.byte_length
    }

    pub fn digest(&self) -> &EvidenceDigest {
        &self.digest
    }

    fn validate(&self) -> Result<(), BackendEvidenceError> {
        if self.relative_path.is_empty()
            || self.relative_path.starts_with('/')
            || self.relative_path.contains('\\')
            || self
                .relative_path
                .split('/')
                .any(|component| component.is_empty() || component == "." || component == "..")
        {
            return Err(BackendEvidenceError::Invariant(
                "Cubical source path is not a canonical relative path",
            ));
        }
        Ok(())
    }

    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.text(&self.relative_path);
        encoder.u64(self.byte_length);
        encoder.text(self.digest.as_str());
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CubicalSourceTreeEvidence {
    contract_id: String,
    expected_digest: EvidenceDigest,
    observed_digest: EvidenceDigest,
    source_file_count: u64,
    source_byte_length: u64,
    files: Vec<CubicalSourceFileEvidence>,
}

impl CubicalSourceTreeEvidence {
    pub(crate) fn new(
        expected_digest: EvidenceDigest,
        files: Vec<CubicalSourceFileEvidence>,
    ) -> Result<Self, BackendEvidenceError> {
        Self::new_for_contract(CUBICAL_SOURCE_TREE_CONTRACT_ID, expected_digest, files)
    }

    pub(crate) fn new_agda_primitive(
        expected_digest: EvidenceDigest,
        files: Vec<CubicalSourceFileEvidence>,
    ) -> Result<Self, BackendEvidenceError> {
        Self::new_for_contract(
            AGDA_PRIMITIVE_SOURCE_TREE_CONTRACT_ID,
            expected_digest,
            files,
        )
    }

    fn new_for_contract(
        contract_id: &'static str,
        expected_digest: EvidenceDigest,
        files: Vec<CubicalSourceFileEvidence>,
    ) -> Result<Self, BackendEvidenceError> {
        let source_file_count = files.len() as u64;
        let source_byte_length = files.iter().try_fold(0_u64, |total, file| {
            total
                .checked_add(file.byte_length)
                .ok_or(BackendEvidenceError::Invariant(
                    "Cubical source byte count overflowed",
                ))
        })?;
        let observed_digest = source_tree_digest(contract_id, &files);
        let evidence = Self {
            contract_id: contract_id.to_owned(),
            expected_digest,
            observed_digest,
            source_file_count,
            source_byte_length,
            files,
        };
        evidence.validate()?;
        Ok(evidence)
    }

    pub fn contract_id(&self) -> &str {
        &self.contract_id
    }

    pub fn expected_digest(&self) -> &EvidenceDigest {
        &self.expected_digest
    }

    pub fn observed_digest(&self) -> &EvidenceDigest {
        &self.observed_digest
    }

    pub fn source_file_count(&self) -> u64 {
        self.source_file_count
    }

    pub fn source_byte_length(&self) -> u64 {
        self.source_byte_length
    }

    pub fn files(&self) -> &[CubicalSourceFileEvidence] {
        &self.files
    }

    fn validate(&self) -> Result<(), BackendEvidenceError> {
        if !matches!(
            self.contract_id.as_str(),
            CUBICAL_SOURCE_TREE_CONTRACT_ID | AGDA_PRIMITIVE_SOURCE_TREE_CONTRACT_ID
        ) {
            return Err(BackendEvidenceError::Invariant(
                "source-tree evidence has an unknown contract identifier",
            ));
        }
        if self.expected_digest != self.observed_digest {
            return Err(BackendEvidenceError::Invariant(
                "Cubical source tree does not match the reviewed digest pin",
            ));
        }
        if self.source_file_count != self.files.len() as u64 || self.files.is_empty() {
            return Err(BackendEvidenceError::Invariant(
                "Cubical source-file count is inconsistent or empty",
            ));
        }
        let mut total = 0_u64;
        let mut previous: Option<&str> = None;
        for file in &self.files {
            file.validate()?;
            if previous.is_some_and(|path| path >= file.relative_path()) {
                return Err(BackendEvidenceError::Invariant(
                    "Cubical source files are duplicated or out of canonical order",
                ));
            }
            previous = Some(file.relative_path());
            total = total
                .checked_add(file.byte_length)
                .ok_or(BackendEvidenceError::Invariant(
                    "Cubical source byte count overflowed",
                ))?;
        }
        if total != self.source_byte_length {
            return Err(BackendEvidenceError::Invariant(
                "Cubical source byte count is inconsistent",
            ));
        }
        if self.observed_digest != source_tree_digest(&self.contract_id, &self.files) {
            return Err(BackendEvidenceError::Invariant(
                "source tree digest does not match its file manifest",
            ));
        }
        Ok(())
    }

    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.text(&self.contract_id);
        encoder.text(self.expected_digest.as_str());
        encoder.text(self.observed_digest.as_str());
        encoder.u64(self.source_file_count);
        encoder.u64(self.source_byte_length);
        encoder.u64(self.files.len() as u64);
        for file in &self.files {
            file.encode_canonical(encoder);
        }
    }
}

pub(crate) fn source_tree_digest(
    contract_id: &str,
    files: &[CubicalSourceFileEvidence],
) -> EvidenceDigest {
    let mut encoder = CanonicalEncoder::new();
    encoder.text(contract_id);
    encoder.u64(files.len() as u64);
    for file in files {
        file.encode_canonical(&mut encoder);
    }
    EvidenceDigest::of_bytes(encoder.as_bytes())
}

fn require_reviewed_source_tree_digest(
    evidence: &CubicalSourceTreeEvidence,
    reviewed_digest: &str,
    test_fixture_path: &str,
    test_fixture_bytes: &[u8],
    message: &'static str,
) -> Result<(), BackendEvidenceError> {
    if evidence.expected_digest().as_str() == reviewed_digest {
        return Ok(());
    }
    #[cfg(test)]
    {
        let fixture_files = vec![CubicalSourceFileEvidence::new(
            test_fixture_path.to_owned(),
            test_fixture_bytes,
        )];
        let fixture_digest = source_tree_digest(evidence.contract_id(), &fixture_files);
        if evidence.expected_digest() == &fixture_digest {
            return Ok(());
        }
    }
    #[cfg(not(test))]
    {
        let _ = (test_fixture_path, test_fixture_bytes);
    }
    Err(BackendEvidenceError::Invariant(message))
}

pub type AgdaPrimitiveSourceFileEvidence = CubicalSourceFileEvidence;
pub type AgdaPrimitiveSourceTreeEvidence = CubicalSourceTreeEvidence;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CubicalCheckoutEvidence {
    root_path: String,
    snapshot_source_root_path: String,
    expected_commit: String,
    observed_commit: String,
    git_executable_path: String,
    expected_git_executable_digest: EvidenceDigest,
    observed_git_executable_digest: EvidenceDigest,
    source_tree: CubicalSourceTreeEvidence,
    library_file_name: String,
    library_file_byte_length: u64,
    library_file_digest: EvidenceDigest,
    library_name: String,
    library_include: String,
    library_flags: Vec<String>,
    top_level_probe: ProcessEvidence,
    head_before_probe: ProcessEvidence,
    head_after_probe: ProcessEvidence,
}

impl CubicalCheckoutEvidence {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        root_path: String,
        snapshot_source_root_path: String,
        observed_commit: String,
        git_executable_path: String,
        expected_git_executable_digest: EvidenceDigest,
        observed_git_executable_digest: EvidenceDigest,
        source_tree: CubicalSourceTreeEvidence,
        library_file_byte_length: u64,
        library_file_digest: EvidenceDigest,
        top_level_probe: ProcessEvidence,
        head_before_probe: ProcessEvidence,
        head_after_probe: ProcessEvidence,
    ) -> Self {
        Self {
            root_path,
            snapshot_source_root_path,
            expected_commit: CUBICAL_EXPECTED_COMMIT.to_owned(),
            observed_commit,
            git_executable_path,
            expected_git_executable_digest,
            observed_git_executable_digest,
            source_tree,
            library_file_name: "cubical.agda-lib".to_owned(),
            library_file_byte_length,
            library_file_digest,
            library_name: CUBICAL_EXPECTED_LIBRARY_NAME.to_owned(),
            library_include: CUBICAL_EXPECTED_INCLUDE.to_owned(),
            library_flags: CUBICAL_EXPECTED_FLAGS
                .iter()
                .map(|flag| (*flag).to_owned())
                .collect(),
            top_level_probe,
            head_before_probe,
            head_after_probe,
        }
    }

    pub fn root_path(&self) -> &str {
        &self.root_path
    }

    pub fn snapshot_source_root_path(&self) -> &str {
        &self.snapshot_source_root_path
    }

    pub fn expected_commit(&self) -> &str {
        &self.expected_commit
    }

    pub fn observed_commit(&self) -> &str {
        &self.observed_commit
    }

    pub fn git_executable_path(&self) -> &str {
        &self.git_executable_path
    }

    pub fn expected_git_executable_digest(&self) -> &EvidenceDigest {
        &self.expected_git_executable_digest
    }

    pub fn observed_git_executable_digest(&self) -> &EvidenceDigest {
        &self.observed_git_executable_digest
    }

    pub fn source_tree(&self) -> &CubicalSourceTreeEvidence {
        &self.source_tree
    }

    pub fn library_file_digest(&self) -> &EvidenceDigest {
        &self.library_file_digest
    }

    pub fn library_flags(&self) -> &[String] {
        &self.library_flags
    }

    fn validate(&self) -> Result<(), BackendEvidenceError> {
        require_absolute_path(&self.root_path, "Cubical root path is not absolute")?;
        require_absolute_path(
            &self.snapshot_source_root_path,
            "Cubical snapshot source-root path is not absolute",
        )?;
        require_absolute_path(
            &self.git_executable_path,
            "Git executable path is not absolute",
        )?;
        if self.expected_commit != CUBICAL_EXPECTED_COMMIT
            || self.observed_commit != CUBICAL_EXPECTED_COMMIT
        {
            return Err(BackendEvidenceError::Invariant(
                "Cubical evidence does not match the exact commit pin",
            ));
        }
        if self.expected_git_executable_digest != self.observed_git_executable_digest {
            return Err(BackendEvidenceError::Invariant(
                "Git executable bytes do not match the trusted digest pin",
            ));
        }
        if self.source_tree.contract_id() != CUBICAL_SOURCE_TREE_CONTRACT_ID {
            return Err(BackendEvidenceError::Invariant(
                "Cubical source tree uses the wrong evidence contract",
            ));
        }
        self.source_tree.validate()?;
        require_reviewed_source_tree_digest(
            &self.source_tree,
            CUBICAL_EXPECTED_SOURCE_TREE_DIGEST,
            "Cubical/Foundations/Prelude.agda",
            b"module Cubical.Foundations.Prelude where\n",
            "Cubical source tree does not match the reviewed digest pin",
        )?;
        if self.library_file_name != "cubical.agda-lib"
            || self.library_name != CUBICAL_EXPECTED_LIBRARY_NAME
            || self.library_include != CUBICAL_EXPECTED_INCLUDE
            || self.library_flags
                != CUBICAL_EXPECTED_FLAGS
                    .iter()
                    .map(|flag| (*flag).to_owned())
                    .collect::<Vec<_>>()
        {
            return Err(BackendEvidenceError::Invariant(
                "Cubical library evidence does not match the exact library pin",
            ));
        }
        for process in [
            &self.top_level_probe,
            &self.head_before_probe,
            &self.head_after_probe,
        ] {
            process.validate()?;
            if process.exit_code != 0 {
                return Err(BackendEvidenceError::Invariant(
                    "a Git audit command was not successful",
                ));
            }
        }
        let prefix = expected_git_prefix(&self.root_path);
        require_arguments(
            &self.top_level_probe,
            extend_arguments(&prefix, &["rev-parse", "--show-toplevel"]),
            "Git top-level audit arguments are not canonical",
        )?;
        require_arguments(
            &self.head_before_probe,
            extend_arguments(&prefix, &["rev-parse", "HEAD"]),
            "Git pre-check HEAD arguments are not canonical",
        )?;
        require_arguments(
            &self.head_after_probe,
            extend_arguments(&prefix, &["rev-parse", "HEAD"]),
            "Git post-check HEAD arguments are not canonical",
        )?;
        if self.head_before_probe.stdout.text().trim() != CUBICAL_EXPECTED_COMMIT
            || self.head_after_probe.stdout.text().trim() != CUBICAL_EXPECTED_COMMIT
        {
            return Err(BackendEvidenceError::Invariant(
                "captured Git HEAD output does not match the commit pin",
            ));
        }
        let normalized_library = CUBICAL_LIBRARY_NORMALIZED.as_bytes();
        if self.library_file_byte_length != normalized_library.len() as u64
            || self.library_file_digest != EvidenceDigest::of_bytes(normalized_library)
        {
            return Err(BackendEvidenceError::Invariant(
                "normalized Cubical library byte evidence does not match the exact pin",
            ));
        }
        let reported_top_level = self.top_level_probe.stdout.text().trim();
        if !equivalent_absolute_path_text(reported_top_level, &self.root_path) {
            return Err(BackendEvidenceError::Invariant(
                "captured Git top-level output disagrees with the checkout root",
            ));
        }
        Ok(())
    }

    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.text(&self.root_path);
        encoder.text(&self.snapshot_source_root_path);
        encoder.text(&self.expected_commit);
        encoder.text(&self.observed_commit);
        encoder.text(&self.git_executable_path);
        encoder.text(self.expected_git_executable_digest.as_str());
        encoder.text(self.observed_git_executable_digest.as_str());
        self.source_tree.encode_canonical(encoder);
        encoder.text(&self.library_file_name);
        encoder.u64(self.library_file_byte_length);
        encoder.text(self.library_file_digest.as_str());
        encoder.text(&self.library_name);
        encoder.text(&self.library_include);
        encoder.text_sequence(&self.library_flags);
        self.top_level_probe.encode_canonical(encoder);
        self.head_before_probe.encode_canonical(encoder);
        self.head_after_probe.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckerInvocationEvidence {
    working_directory: String,
    source_include_path: String,
    source_path: String,
    cubical_include_path: String,
    agda_data_directory_path: String,
    timeout_millis: u64,
    process: ProcessEvidence,
}

impl CheckerInvocationEvidence {
    pub(crate) fn new(
        working_directory: String,
        source_path: String,
        cubical_include_path: String,
        agda_data_directory_path: String,
        timeout_millis: u64,
        process: ProcessEvidence,
    ) -> Self {
        Self {
            source_include_path: working_directory.clone(),
            working_directory,
            source_path,
            cubical_include_path,
            agda_data_directory_path,
            timeout_millis,
            process,
        }
    }

    pub fn working_directory(&self) -> &str {
        &self.working_directory
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn cubical_include_path(&self) -> &str {
        &self.cubical_include_path
    }

    pub fn agda_data_directory_path(&self) -> &str {
        &self.agda_data_directory_path
    }

    pub fn timeout_millis(&self) -> u64 {
        self.timeout_millis
    }

    pub fn process(&self) -> &ProcessEvidence {
        &self.process
    }

    fn validate(&self) -> Result<(), BackendEvidenceError> {
        require_absolute_path(
            &self.working_directory,
            "checker working directory is not absolute",
        )?;
        require_absolute_path(&self.source_path, "checker source path is not absolute")?;
        require_absolute_path(
            &self.cubical_include_path,
            "checker Cubical include path is not absolute",
        )?;
        require_absolute_path(
            &self.agda_data_directory_path,
            "checker Agda data-directory path is not absolute",
        )?;
        if self.source_include_path != self.working_directory {
            return Err(BackendEvidenceError::Invariant(
                "checker source include and working directory differ",
            ));
        }
        if Path::new(&self.source_path)
            .file_name()
            .and_then(|name| name.to_str())
            != Some(SMOKE_FILE_NAME)
            || Path::new(&self.source_path).parent() != Some(Path::new(&self.source_include_path))
        {
            return Err(BackendEvidenceError::Invariant(
                "checker source path does not identify the fixed smoke file",
            ));
        }
        if self.timeout_millis == 0 || self.timeout_millis > MAX_CHECKER_TIMEOUT_MILLIS {
            return Err(BackendEvidenceError::Invariant(
                "checker timeout is outside the fixed positive bound",
            ));
        }
        self.process.validate()?;
        if self.process.exit_code != 0 {
            return Err(BackendEvidenceError::Invariant(
                "checker process was not successful",
            ));
        }
        let expected = expected_checker_arguments(
            &self.source_include_path,
            &self.cubical_include_path,
            &self.source_path,
        );
        if self.process.arguments != expected {
            return Err(BackendEvidenceError::Invariant(
                "checker arguments are not the canonical explicit safe/cubical invocation",
            ));
        }
        Ok(())
    }

    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.text(&self.working_directory);
        encoder.text(&self.source_include_path);
        encoder.text(&self.source_path);
        encoder.text(&self.cubical_include_path);
        encoder.text(&self.agda_data_directory_path);
        encoder.u64(self.timeout_millis);
        self.process.encode_canonical(encoder);
    }
}

/// Strict, versioned evidence for a successful pinned readiness probe.
///
/// Deserializing this value validates every fixed pin and the canonical
/// digest. It still does not create [`crate::VerifiedPinnedBackend`].
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackendEvidenceManifest {
    schema_version: u16,
    backend_contract: String,
    source: FixedSmokeSourceEvidence,
    agda: AgdaToolchainEvidence,
    cubical: CubicalCheckoutEvidence,
    checker: CheckerInvocationEvidence,
    canonical_digest: EvidenceDigest,
}

impl BackendEvidenceManifest {
    pub(crate) fn new(
        agda: AgdaToolchainEvidence,
        cubical: CubicalCheckoutEvidence,
        checker: CheckerInvocationEvidence,
    ) -> Result<Self, BackendEvidenceError> {
        let mut manifest = Self {
            schema_version: EVIDENCE_SCHEMA_VERSION,
            backend_contract: BACKEND_CONTRACT_ID.to_owned(),
            source: FixedSmokeSourceEvidence::current(),
            agda,
            cubical,
            checker,
            canonical_digest: EvidenceDigest::of_bytes(b"pending canonical evidence digest"),
        };
        manifest.canonical_digest = manifest.computed_digest();
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn backend_contract(&self) -> &str {
        &self.backend_contract
    }

    pub fn source(&self) -> &FixedSmokeSourceEvidence {
        &self.source
    }

    pub fn agda(&self) -> &AgdaToolchainEvidence {
        &self.agda
    }

    pub fn cubical(&self) -> &CubicalCheckoutEvidence {
        &self.cubical
    }

    pub fn checker(&self) -> &CheckerInvocationEvidence {
        &self.checker
    }

    pub fn canonical_digest(&self) -> &EvidenceDigest {
        &self.canonical_digest
    }

    pub fn validate(&self) -> Result<(), BackendEvidenceError> {
        if self.schema_version != EVIDENCE_SCHEMA_VERSION {
            return Err(BackendEvidenceError::UnsupportedSchemaVersion {
                expected: EVIDENCE_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.backend_contract != BACKEND_CONTRACT_ID {
            return Err(BackendEvidenceError::Invariant(
                "backend contract identifier does not match the versioned contract",
            ));
        }
        self.source.validate()?;
        self.agda.validate()?;
        self.cubical.validate()?;
        self.checker.validate()?;
        if self.checker.cubical_include_path != self.cubical.snapshot_source_root_path {
            return Err(BackendEvidenceError::Invariant(
                "checker Cubical include path disagrees with the verified snapshot",
            ));
        }
        if self.checker.agda_data_directory_path
            != self.agda.data_directory.snapshot_data_directory_path
        {
            return Err(BackendEvidenceError::Invariant(
                "checker Agda data directory disagrees with the verified snapshot",
            ));
        }
        for (path, expected_name, message) in [
            (
                self.cubical.snapshot_source_root_path.as_str(),
                CUBICAL_SNAPSHOT_DIRECTORY_NAME,
                "Cubical snapshot is not the fixed child of the checker scratch directory",
            ),
            (
                self.agda
                    .data_directory
                    .snapshot_data_directory_path
                    .as_str(),
                AGDA_DATA_SNAPSHOT_DIRECTORY_NAME,
                "Agda data snapshot is not the fixed child of the checker scratch directory",
            ),
        ] {
            let path = Path::new(path);
            if path.file_name().and_then(|name| name.to_str()) != Some(expected_name)
                || path.parent() != Some(Path::new(&self.checker.working_directory))
            {
                return Err(BackendEvidenceError::Invariant(message));
            }
        }
        let computed = self.computed_digest();
        if self.canonical_digest != computed {
            return Err(BackendEvidenceError::CanonicalDigestMismatch {
                expected: computed,
                actual: self.canonical_digest.clone(),
            });
        }
        Ok(())
    }

    fn computed_digest(&self) -> EvidenceDigest {
        let mut encoder = CanonicalEncoder::new();
        encoder.text("pen-gf2-agda/backend-evidence/canonical/v3");
        encoder.u16(self.schema_version);
        encoder.text(&self.backend_contract);
        self.source.encode_canonical(&mut encoder);
        self.agda.encode_canonical(&mut encoder);
        self.cubical.encode_canonical(&mut encoder);
        self.checker.encode_canonical(&mut encoder);
        EvidenceDigest::of_bytes(encoder.as_bytes())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BackendEvidenceManifestWire {
    schema_version: u16,
    backend_contract: String,
    source: FixedSmokeSourceEvidence,
    agda: AgdaToolchainEvidence,
    cubical: CubicalCheckoutEvidence,
    checker: CheckerInvocationEvidence,
    canonical_digest: EvidenceDigest,
}

impl<'de> Deserialize<'de> for BackendEvidenceManifest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = BackendEvidenceManifestWire::deserialize(deserializer)?;
        let manifest = Self {
            schema_version: wire.schema_version,
            backend_contract: wire.backend_contract,
            source: wire.source,
            agda: wire.agda,
            cubical: wire.cubical,
            checker: wire.checker,
            canonical_digest: wire.canonical_digest,
        };
        manifest.validate().map_err(de::Error::custom)?;
        Ok(manifest)
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum BackendEvidenceError {
    #[error("unsupported evidence schema version {actual}; expected {expected}")]
    UnsupportedSchemaVersion { expected: u16, actual: u16 },
    #[error("invalid backend evidence: {0}")]
    Invariant(&'static str),
    #[error("canonical evidence digest mismatch: expected {expected}, found {actual}")]
    CanonicalDigestMismatch {
        expected: EvidenceDigest,
        actual: EvidenceDigest,
    },
}

pub(crate) fn expected_checker_arguments(
    source_include_path: &str,
    cubical_include_path: &str,
    source_path: &str,
) -> Vec<String> {
    let mut arguments = vec![
        "--no-libraries".to_owned(),
        "--ignore-interfaces".to_owned(),
    ];
    arguments.extend(
        CUBICAL_EXPECTED_FLAGS
            .iter()
            .map(|argument| (*argument).to_owned()),
    );
    arguments.extend([
        "-i".to_owned(),
        source_include_path.to_owned(),
        "-i".to_owned(),
        cubical_include_path.to_owned(),
        source_path.to_owned(),
    ]);
    arguments
}

pub(crate) fn expected_git_prefix(root_path: &str) -> Vec<String> {
    [
        "--no-pager",
        "-c",
        "core.fsmonitor=false",
        "-c",
        "core.hooksPath=/dev/null",
        "-c",
        "color.ui=false",
        "-C",
        root_path,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

pub(crate) fn parse_agda_version(stdout: &str) -> Option<String> {
    let first_line = stdout.lines().next()?.trim_end_matches('\r');
    first_line.strip_prefix("Agda version ").map(str::to_owned)
}

fn require_absolute_path(path: &str, message: &'static str) -> Result<(), BackendEvidenceError> {
    if Path::new(path).is_absolute() {
        Ok(())
    } else {
        Err(BackendEvidenceError::Invariant(message))
    }
}

#[cfg(windows)]
fn equivalent_absolute_path_text(left: &str, right: &str) -> bool {
    fn normalized(value: &str) -> String {
        let replaced = value.replace('/', "\\");
        let without_extended_prefix = replaced.strip_prefix(r"\\?\").unwrap_or(&replaced);
        without_extended_prefix
            .trim_end_matches('\\')
            .to_ascii_lowercase()
    }
    Path::new(left).is_absolute()
        && Path::new(right).is_absolute()
        && normalized(left) == normalized(right)
}

#[cfg(not(windows))]
fn equivalent_absolute_path_text(left: &str, right: &str) -> bool {
    Path::new(left).is_absolute()
        && Path::new(right).is_absolute()
        && Path::new(left) == Path::new(right)
}

fn require_arguments(
    process: &ProcessEvidence,
    expected: Vec<String>,
    message: &'static str,
) -> Result<(), BackendEvidenceError> {
    if process.arguments == expected {
        Ok(())
    } else {
        Err(BackendEvidenceError::Invariant(message))
    }
}

fn extend_arguments(prefix: &[String], suffix: &[&str]) -> Vec<String> {
    prefix
        .iter()
        .cloned()
        .chain(suffix.iter().map(|item| (*item).to_owned()))
        .collect()
}

#[derive(Default)]
struct CanonicalEncoder {
    bytes: Vec<u8>,
}

impl CanonicalEncoder {
    fn new() -> Self {
        Self::default()
    }

    fn u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn i32(&mut self, value: i32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn text(&mut self, value: &str) {
        self.u64(value.len() as u64);
        self.bytes.extend_from_slice(value.as_bytes());
    }

    fn text_sequence(&mut self, values: &[String]) {
        self.u64(values.len() as u64);
        for value in values {
            self.text(value);
        }
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AGDA_EXPECTED_VERSION, AGDA_REFERENCE_PRIMITIVE_TREE_DIGEST, AgdaDataDirectoryEvidence,
        AgdaToolchainEvidence, BackendEvidenceError, BackendEvidenceManifest, CapturedOutput,
        CheckerInvocationEvidence, CubicalCheckoutEvidence, CubicalSourceFileEvidence,
        CubicalSourceTreeEvidence, EVIDENCE_SCHEMA_VERSION, EvidenceDigest,
        FixedSmokeSourceEvidence, MAX_CHECKER_TIMEOUT_MILLIS, ProcessEvidence, SMOKE_SOURCE,
        SMOKE_SOURCE_DIGEST, expected_checker_arguments,
    };
    use serde_json::{Value, json};
    use std::path::PathBuf;

    fn absolute_fixture_root() -> String {
        std::env::current_dir()
            .expect("current directory")
            .join("fixture-cubical")
            .to_string_lossy()
            .into_owned()
    }

    fn fixture_manifest() -> BackendEvidenceManifest {
        let cubical_root = absolute_fixture_root();
        let scratch = std::env::current_dir()
            .expect("current directory")
            .join("fixture-smoke");
        let working_directory = scratch.to_string_lossy().into_owned();
        let cubical_snapshot = scratch
            .join(super::CUBICAL_SNAPSHOT_DIRECTORY_NAME)
            .to_string_lossy()
            .into_owned();
        let agda_data_snapshot = scratch
            .join(super::AGDA_DATA_SNAPSHOT_DIRECTORY_NAME)
            .to_string_lossy()
            .into_owned();
        let installed_agda_data = std::env::current_dir()
            .expect("current directory")
            .join("fixture-agda-data")
            .to_string_lossy()
            .into_owned();
        let source_path = scratch
            .join(super::SMOKE_FILE_NAME)
            .to_string_lossy()
            .into_owned();
        let git_path = std::env::current_dir()
            .expect("current directory")
            .join("git")
            .to_string_lossy()
            .into_owned();
        let agda_path = std::env::current_dir()
            .expect("current directory")
            .join("agda")
            .to_string_lossy()
            .into_owned();
        let empty = || CapturedOutput::new(String::new());
        let git = |suffix: &[&str], stdout: String| {
            ProcessEvidence::new(
                super::expected_git_prefix(&cubical_root)
                    .into_iter()
                    .chain(suffix.iter().map(|item| (*item).to_owned()))
                    .collect(),
                0,
                CapturedOutput::new(stdout),
                empty(),
            )
        };
        let primitive_source_files = vec![CubicalSourceFileEvidence::new(
            "Agda/Primitive.agda".to_owned(),
            b"module Agda.Primitive where\n",
        )];
        let primitive_source_tree_digest = super::source_tree_digest(
            super::AGDA_PRIMITIVE_SOURCE_TREE_CONTRACT_ID,
            &primitive_source_files,
        );
        let primitive_source_tree = CubicalSourceTreeEvidence::new_agda_primitive(
            primitive_source_tree_digest,
            primitive_source_files,
        )
        .expect("valid primitive source tree fixture");
        let agda = AgdaToolchainEvidence::new(
            AGDA_EXPECTED_VERSION.to_owned(),
            agda_path,
            EvidenceDigest::of_bytes(b"agda"),
            EvidenceDigest::of_bytes(b"agda"),
            AgdaDataDirectoryEvidence::new(
                installed_agda_data.clone(),
                agda_data_snapshot.clone(),
                primitive_source_tree,
                ProcessEvidence::new(
                    vec!["--print-agda-data-dir".to_owned()],
                    0,
                    CapturedOutput::new(format!("{installed_agda_data}\n")),
                    empty(),
                ),
            ),
            ProcessEvidence::new(
                vec!["--version".to_owned()],
                0,
                CapturedOutput::new(format!(
                    "Agda version {AGDA_EXPECTED_VERSION}\nfixture build\n"
                )),
                empty(),
            ),
        );
        let source_files = vec![CubicalSourceFileEvidence::new(
            "Cubical/Foundations/Prelude.agda".to_owned(),
            b"module Cubical.Foundations.Prelude where\n",
        )];
        let source_tree_digest =
            super::source_tree_digest(super::CUBICAL_SOURCE_TREE_CONTRACT_ID, &source_files);
        let source_tree = CubicalSourceTreeEvidence::new(source_tree_digest, source_files)
            .expect("valid source tree fixture");
        let cubical = CubicalCheckoutEvidence::new(
            cubical_root.clone(),
            cubical_snapshot.clone(),
            super::CUBICAL_EXPECTED_COMMIT.to_owned(),
            git_path,
            EvidenceDigest::of_bytes(b"git"),
            EvidenceDigest::of_bytes(b"git"),
            source_tree,
            super::CUBICAL_LIBRARY_NORMALIZED.len() as u64,
            EvidenceDigest::of_bytes(super::CUBICAL_LIBRARY_NORMALIZED.as_bytes()),
            git(
                &["rev-parse", "--show-toplevel"],
                format!("{cubical_root}\n"),
            ),
            git(
                &["rev-parse", "HEAD"],
                format!("{}\n", super::CUBICAL_EXPECTED_COMMIT),
            ),
            git(
                &["rev-parse", "HEAD"],
                format!("{}\n", super::CUBICAL_EXPECTED_COMMIT),
            ),
        );
        let checker = CheckerInvocationEvidence::new(
            working_directory.clone(),
            source_path.clone(),
            cubical_snapshot.clone(),
            agda_data_snapshot,
            30_000,
            ProcessEvidence::new(
                expected_checker_arguments(&working_directory, &cubical_snapshot, &source_path),
                0,
                empty(),
                empty(),
            ),
        );
        BackendEvidenceManifest::new(agda, cubical, checker).expect("valid fixture")
    }

    #[test]
    fn fixed_source_digest_is_snapshotted_and_contains_no_postulate() {
        assert_eq!(
            EvidenceDigest::of_bytes(SMOKE_SOURCE.as_bytes()).as_str(),
            SMOKE_SOURCE_DIGEST
        );
        assert!(!SMOKE_SOURCE.to_lowercase().contains("postulate"));
        assert_eq!(
            FixedSmokeSourceEvidence::current().module_name(),
            super::SMOKE_MODULE_NAME
        );
    }

    #[test]
    fn canonical_manifest_round_trips() {
        let manifest = fixture_manifest();
        let wire = serde_json::to_string_pretty(&manifest).expect("serialize");
        let decoded: BackendEvidenceManifest =
            serde_json::from_str(&wire).expect("strict deserialize");
        assert_eq!(decoded, manifest);
        assert_eq!(decoded.validate(), Ok(()));
    }

    #[test]
    fn unknown_fields_are_rejected_at_every_relevant_boundary() {
        let manifest = fixture_manifest();
        let mut top = serde_json::to_value(&manifest).expect("value");
        top.as_object_mut()
            .expect("object")
            .insert("authority".to_owned(), json!("forged"));
        assert!(serde_json::from_value::<BackendEvidenceManifest>(top).is_err());

        let mut nested = serde_json::to_value(&manifest).expect("value");
        nested["checker"]["process"]
            .as_object_mut()
            .expect("process object")
            .insert("accepted_source".to_owned(), json!("caller supplied"));
        assert!(serde_json::from_value::<BackendEvidenceManifest>(nested).is_err());
    }

    #[test]
    fn pin_argument_and_output_mutations_are_rejected() {
        let manifest = fixture_manifest();
        for (pointer, replacement) in [
            ("/schema_version", json!(EVIDENCE_SCHEMA_VERSION + 1)),
            ("/agda/observed_version", json!("2.8.1")),
            (
                "/agda/observed_executable_digest",
                json!(EvidenceDigest::of_bytes(b"forged-agda")),
            ),
            (
                "/agda/data_directory/primitive_source_tree/contract_id",
                json!("pen-gf2-agda/forged-tree/v1"),
            ),
            (
                "/cubical/observed_commit",
                json!("0000000000000000000000000000000000000000"),
            ),
            (
                "/cubical/observed_git_executable_digest",
                json!(EvidenceDigest::of_bytes(b"forged-git")),
            ),
            (
                "/cubical/source_tree/files/0/digest",
                json!(EvidenceDigest::of_bytes(b"forged-source")),
            ),
            (
                "/checker/process/arguments/0",
                json!("--allow-unsolved-metas"),
            ),
            (
                "/checker/timeout_millis",
                json!(MAX_CHECKER_TIMEOUT_MILLIS + 1),
            ),
            (
                "/canonical_digest",
                json!(format!("blake3:{}", "0".repeat(64))),
            ),
        ] {
            let mut value = serde_json::to_value(&manifest).expect("value");
            *value.pointer_mut(pointer).expect("fixture pointer") = replacement;
            assert!(
                serde_json::from_value::<BackendEvidenceManifest>(value).is_err(),
                "mutation at {pointer} must fail"
            );
        }
    }

    #[test]
    fn fixed_invocation_has_explicit_safe_cubical_and_include_options() {
        let source_root = PathBuf::from("C:/scratch").to_string_lossy().into_owned();
        let cubical_root = PathBuf::from("C:/cubical").to_string_lossy().into_owned();
        let source = PathBuf::from(&source_root)
            .join(super::SMOKE_FILE_NAME)
            .to_string_lossy()
            .into_owned();
        let arguments = expected_checker_arguments(&source_root, &cubical_root, &source);
        assert_eq!(
            arguments,
            [
                "--no-libraries",
                "--ignore-interfaces",
                "--safe",
                "--cubical",
                "--no-import-sorts",
                "-WnoUnsupportedIndexedMatch",
                "--guardedness",
                "-i",
                source_root.as_str(),
                "-i",
                cubical_root.as_str(),
                source.as_str(),
            ]
        );
        assert!(!arguments.iter().any(|argument| argument == "--library"));
    }

    #[test]
    fn fixed_git_prefix_disables_hooks_fsmonitor_and_paging() {
        let root = PathBuf::from("C:/cubical").to_string_lossy().into_owned();
        assert_eq!(
            super::expected_git_prefix(&root),
            [
                "--no-pager",
                "-c",
                "core.fsmonitor=false",
                "-c",
                "core.hooksPath=/dev/null",
                "-c",
                "color.ui=false",
                "-C",
                root.as_str(),
            ]
        );
    }

    #[test]
    fn tampering_with_captured_text_cannot_be_hidden_by_old_digest() {
        let manifest = fixture_manifest();
        let mut value: Value = serde_json::to_value(&manifest).expect("value");
        value["agda"]["version_probe"]["stdout"]["text"] =
            json!("Agda version 2.8.0\nmalicious replacement\n");
        assert!(serde_json::from_value::<BackendEvidenceManifest>(value).is_err());
    }

    #[test]
    fn recomputed_digest_cannot_hide_fixed_library_cubical_or_top_level_forgery() {
        let mut bad_library = fixture_manifest();
        bad_library.cubical.library_file_byte_length = 1;
        bad_library.cubical.library_file_digest = EvidenceDigest::of_bytes(b"x");
        bad_library.canonical_digest = bad_library.computed_digest();
        assert!(bad_library.validate().is_err());

        let mut bad_top_level = fixture_manifest();
        bad_top_level.cubical.top_level_probe.stdout =
            CapturedOutput::new(format!("{}-other\n", absolute_fixture_root()));
        bad_top_level.canonical_digest = bad_top_level.computed_digest();
        assert!(bad_top_level.validate().is_err());

        let mut bad_cubical_tree = fixture_manifest();
        let alternate_cubical_files = vec![CubicalSourceFileEvidence::new(
            "Cubical/Foundations/Prelude.agda".to_owned(),
            b"module Cubical.Foundations.Prelude where\nforged = 1\n",
        )];
        let alternate_cubical_digest = super::source_tree_digest(
            super::CUBICAL_SOURCE_TREE_CONTRACT_ID,
            &alternate_cubical_files,
        );
        bad_cubical_tree.cubical.source_tree =
            CubicalSourceTreeEvidence::new(alternate_cubical_digest, alternate_cubical_files)
                .expect("internally consistent alternate Cubical tree");
        bad_cubical_tree.canonical_digest = bad_cubical_tree.computed_digest();
        assert!(bad_cubical_tree.validate().is_err());
    }

    #[test]
    fn recomputed_digest_cannot_hide_timeout_above_fixed_bound() {
        let mut manifest = fixture_manifest();
        manifest.checker.timeout_millis = MAX_CHECKER_TIMEOUT_MILLIS + 1;
        manifest.canonical_digest = manifest.computed_digest();
        assert_eq!(
            manifest.validate(),
            Err(BackendEvidenceError::Invariant(
                "checker timeout is outside the fixed positive bound"
            ))
        );
    }

    #[test]
    fn primitive_tree_authenticity_requires_the_external_reviewed_pin() {
        let alternate_primitive_files = vec![CubicalSourceFileEvidence::new(
            "Agda/Primitive.agda".to_owned(),
            b"module Agda.Primitive where\nforged = 1\n",
        )];
        let local_reference_pin =
            EvidenceDigest::parse(AGDA_REFERENCE_PRIMITIVE_TREE_DIGEST).expect("reference pin");
        assert!(
            CubicalSourceTreeEvidence::new_agda_primitive(
                local_reference_pin,
                alternate_primitive_files.clone(),
            )
            .is_err(),
            "an independently supplied pin rejects different primitive-runtime bytes"
        );

        // A wire manifest is structural replay evidence, not a capability and
        // not an authenticity root for the caller-supplied distribution pin.
        // Re-basing both a tree and its pin is therefore internally coherent;
        // only `probe_pinned_backend` can mint the non-deserializable verified
        // capability after receiving the trusted pin out of band.
        let alternate_primitive_digest = super::source_tree_digest(
            super::AGDA_PRIMITIVE_SOURCE_TREE_CONTRACT_ID,
            &alternate_primitive_files,
        );
        let mut rebased_manifest = fixture_manifest();
        rebased_manifest.agda.data_directory.primitive_source_tree =
            CubicalSourceTreeEvidence::new_agda_primitive(
                alternate_primitive_digest,
                alternate_primitive_files,
            )
            .expect("internally consistent alternate primitive tree");
        rebased_manifest.canonical_digest = rebased_manifest.computed_digest();
        assert!(rebased_manifest.validate().is_ok());
    }
}
