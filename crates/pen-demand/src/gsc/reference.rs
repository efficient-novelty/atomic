//! Runtime replay of the generic cross-reference vectors.
//!
//! The semantic manifest fixes the ordered suite. This module independently
//! reconstructs its synthetic inputs and exercises the real Rust code and
//! compilers. The returned capability is opaque and cannot be deserialized.

use super::{
    CLOSED_FORMER_FRAME_SCHEMA_VERSION, CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION, ClosedFormerFrame,
    ClosedInductiveCode, CodeTerm, ComputationMode, ConstructorCode, GscOriginEventId, GscOutcome,
    GscReferenceVectorDisposition, GscUnknownReason, IntroductionAlias, PublicSourceId,
    TelescopeCode, VerifiedClosedFormerFrame, VerifiedClosedInductiveCode,
    VerifiedGscSemanticManifest, compile_compute_v1, compile_use_v1, verify_closed_former_frame,
    verify_closed_inductive_code,
};
use pen_gf2_agda::{AGDA_EXPECTED_VERSION, AGDA_REFERENCE_PRIMITIVE_TREE_DIGEST};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Declaration, Digest, GlobalId, Kernel, KernelError,
    OpenJudgment, Term, UncheckedSignature, VerifiedSignature,
};
use std::ffi::OsString;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

const AGDA_EXECUTABLE_DIGEST_PIN: &str =
    "blake3:31f99b2381750bd0387b4695c676f64ca87f370e9ec2f3e98dac76adfab2d32a";
const AGDA_SOURCE_FILE_NAME: &str = "GscInductiveCoreV1.agda";
const AGDA_SOURCE_BYTES: &[u8] = include_bytes!("../../agda/GscInductiveCoreV1.agda");
const AGDA_DATA_SNAPSHOT_DIRECTORY_NAME: &str = "agda-data";
const AGDA_PRIMITIVE_RELATIVE_ROOT: &str = "lib/prim";
const AGDA_PRIMITIVE_SOURCE_TREE_CONTRACT_ID: &str = "pen-gf2-agda/agda-primitive-source-tree/v1";
const AGDA_ARGUMENT_PROTOCOL: &[&str] = &[
    "--no-libraries",
    "--ignore-interfaces",
    "--safe",
    "--without-K",
];
const MAX_AGDA_PRIMITIVE_TREE_ENTRIES: u64 = 100_000;
const MAX_AGDA_PRIMITIVE_SOURCE_FILES: u64 = 20_000;
const MAX_AGDA_PRIMITIVE_SOURCE_FILE_BYTES: u64 = 64 * 1024 * 1024;
const MAX_AGDA_PRIMITIVE_SOURCE_TREE_BYTES: u64 = 1024 * 1024 * 1024;
static SCRATCH_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, Eq, PartialEq)]
struct PrimitiveSourceFileEvidence {
    relative_path: String,
    byte_length: u64,
    digest: Digest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PrimitiveSourceTreeEvidence {
    digest: Digest,
    source_file_count: u64,
    source_byte_length: u64,
    files: Vec<PrimitiveSourceFileEvidence>,
}

struct AgdaGateEvidence {
    primitive_tree: PrimitiveSourceTreeEvidence,
    data_dir_probe_argument_protocol_digest: Digest,
    data_dir_probe_stdout_digest: Digest,
    data_dir_probe_stderr_digest: Digest,
    checker_stdout_digest: Digest,
    checker_stderr_digest: Digest,
}

#[derive(Clone, Debug)]
pub struct VerifiedGscReferenceVectorReplay {
    semantic_manifest_digest: Digest,
    vector_suite_digest: Digest,
    ordered_dispositions: Vec<GscReferenceVectorDisposition>,
    digest: Digest,
}

impl VerifiedGscReferenceVectorReplay {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn vector_suite_digest(&self) -> &Digest {
        &self.vector_suite_digest
    }

    pub fn ordered_dispositions(&self) -> &[GscReferenceVectorDisposition] {
        &self.ordered_dispositions
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Opaque cross-implementation agreement minted only after the real Rust
/// replay and the pinned safe Agda checker invocation both succeed.
#[derive(Clone, Debug)]
pub struct VerifiedGscReferenceAgreement {
    semantic_manifest_digest: Digest,
    vector_suite_digest: Digest,
    rust_replay_digest: Digest,
    agda_source_digest: Digest,
    agda_executable_digest: Digest,
    agda_version: String,
    agda_primitive_tree_digest: Digest,
    agda_primitive_source_file_count: u64,
    agda_primitive_source_byte_length: u64,
    agda_data_dir_probe_argument_protocol_digest: Digest,
    agda_data_dir_probe_stdout_digest: Digest,
    agda_data_dir_probe_stderr_digest: Digest,
    checker_argument_protocol_digest: Digest,
    checker_stdout_digest: Digest,
    checker_stderr_digest: Digest,
    gate_protocol_digest: Digest,
    digest: Digest,
}

impl VerifiedGscReferenceAgreement {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn vector_suite_digest(&self) -> &Digest {
        &self.vector_suite_digest
    }

    pub fn rust_replay_digest(&self) -> &Digest {
        &self.rust_replay_digest
    }

    pub fn agda_source_digest(&self) -> &Digest {
        &self.agda_source_digest
    }

    pub fn agda_executable_digest(&self) -> &Digest {
        &self.agda_executable_digest
    }

    pub fn agda_version(&self) -> &str {
        &self.agda_version
    }

    pub fn agda_primitive_tree_digest(&self) -> &Digest {
        &self.agda_primitive_tree_digest
    }

    pub fn agda_primitive_source_file_count(&self) -> u64 {
        self.agda_primitive_source_file_count
    }

    pub fn agda_primitive_source_byte_length(&self) -> u64 {
        self.agda_primitive_source_byte_length
    }

    pub fn agda_data_dir_probe_argument_protocol_digest(&self) -> &Digest {
        &self.agda_data_dir_probe_argument_protocol_digest
    }

    pub fn agda_data_dir_probe_stdout_digest(&self) -> &Digest {
        &self.agda_data_dir_probe_stdout_digest
    }

    pub fn agda_data_dir_probe_stderr_digest(&self) -> &Digest {
        &self.agda_data_dir_probe_stderr_digest
    }

    pub fn checker_argument_protocol_digest(&self) -> &Digest {
        &self.checker_argument_protocol_digest
    }

    pub fn checker_stdout_digest(&self) -> &Digest {
        &self.checker_stdout_digest
    }

    pub fn checker_stderr_digest(&self) -> &Digest {
        &self.checker_stderr_digest
    }

    pub fn gate_protocol_digest(&self) -> &Digest {
        &self.gate_protocol_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    pub(crate) fn binding_is_valid(&self, semantic: &VerifiedGscSemanticManifest) -> bool {
        self.semantic_manifest_digest == *semantic.digest()
            && self.vector_suite_digest
                == semantic
                    .manifest()
                    .reference_vector_suite
                    .canonical_digest()
            && self.agda_source_digest == super::gsc_agda_reference_digest()
            && self.agda_executable_digest.as_str() == AGDA_EXECUTABLE_DIGEST_PIN
            && self.agda_version == AGDA_EXPECTED_VERSION
            && self.agda_primitive_tree_digest.as_str() == AGDA_REFERENCE_PRIMITIVE_TREE_DIGEST
            && self.agda_primitive_source_file_count > 0
            && self.agda_primitive_source_byte_length > 0
            && self.agda_data_dir_probe_argument_protocol_digest
                == data_dir_probe_argument_protocol_digest()
            && self.agda_data_dir_probe_stdout_digest != Digest::of_bytes(b"")
            && self.agda_data_dir_probe_stderr_digest == Digest::of_bytes(b"")
            && self.checker_argument_protocol_digest == checker_argument_protocol_digest()
            && self.checker_stdout_digest == canonical_checker_stdout_digest()
            && self.checker_stderr_digest == Digest::of_bytes(b"")
            && self.gate_protocol_digest == super::gsc_reference_agreement_gate_digest()
            && self.digest == reference_agreement_digest(self)
    }
}

pub fn verify_gsc_reference_agreement_v1(
    kernel: &Kernel,
    semantic: &VerifiedGscSemanticManifest,
) -> GscOutcome<VerifiedGscReferenceAgreement> {
    let replay = match replay_gsc_reference_vectors_v1(kernel, semantic) {
        GscOutcome::Proven(replay) => replay,
        GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
    };
    let executable = match resolve_agda_executable() {
        Some(executable) => executable,
        None => return GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier),
    };
    let executable_bytes_before = match fs::read(&executable) {
        Ok(bytes) => bytes,
        Err(_) => return GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier),
    };
    let executable_digest = Digest::of_bytes(&executable_bytes_before);
    if executable_digest.as_str() != AGDA_EXECUTABLE_DIGEST_PIN {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier);
    }

    let scratch = match create_scratch_directory() {
        Some(path) => path,
        None => return GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier),
    };
    let gate_evidence = run_pinned_agda_gate(&executable, &scratch);
    let executable_after = fs::read(&executable);
    let cleanup_succeeded = fs::remove_dir_all(&scratch).is_ok();
    let (Some(gate_evidence), Ok(executable_after)) = (gate_evidence, executable_after) else {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier);
    };
    if executable_after != executable_bytes_before || !cleanup_succeeded {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier);
    }

    let agda_source_digest = super::gsc_agda_reference_digest();
    let checker_argument_protocol_digest = checker_argument_protocol_digest();
    let gate_protocol_digest = super::gsc_reference_agreement_gate_digest();
    let mut agreement = VerifiedGscReferenceAgreement {
        semantic_manifest_digest: semantic.digest().clone(),
        vector_suite_digest: replay.vector_suite_digest().clone(),
        rust_replay_digest: replay.digest().clone(),
        agda_source_digest,
        agda_executable_digest: executable_digest,
        agda_version: AGDA_EXPECTED_VERSION.to_owned(),
        agda_primitive_tree_digest: gate_evidence.primitive_tree.digest,
        agda_primitive_source_file_count: gate_evidence.primitive_tree.source_file_count,
        agda_primitive_source_byte_length: gate_evidence.primitive_tree.source_byte_length,
        agda_data_dir_probe_argument_protocol_digest: gate_evidence
            .data_dir_probe_argument_protocol_digest,
        agda_data_dir_probe_stdout_digest: gate_evidence.data_dir_probe_stdout_digest,
        agda_data_dir_probe_stderr_digest: gate_evidence.data_dir_probe_stderr_digest,
        checker_argument_protocol_digest,
        checker_stdout_digest: gate_evidence.checker_stdout_digest,
        checker_stderr_digest: gate_evidence.checker_stderr_digest,
        gate_protocol_digest,
        digest: Digest::of_bytes(b"pending reference agreement"),
    };
    agreement.digest = reference_agreement_digest(&agreement);
    if !agreement.binding_is_valid(semantic) {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier);
    }
    GscOutcome::Proven(agreement)
}

fn run_pinned_agda_gate(executable: &Path, scratch: &Path) -> Option<AgdaGateEvidence> {
    let agda_app = scratch.join("agda-app");
    fs::create_dir(&agda_app).ok()?;

    let mut version_command = Command::new(executable);
    version_command.arg("--version");
    configure_sanitized_environment(&mut version_command, scratch, None);
    let version_output = version_command.output().ok()?;
    let version_stdout = canonical_output(&version_output.stdout)?;
    let version_stderr = canonical_output(&version_output.stderr)?;
    if !version_output.status.success()
        || !version_stderr.trim().is_empty()
        || parse_agda_version(&version_stdout) != Some(AGDA_EXPECTED_VERSION)
    {
        return None;
    }

    let mut data_dir_command = Command::new(executable);
    data_dir_command.arg("--print-agda-data-dir");
    configure_sanitized_environment(&mut data_dir_command, scratch, None);
    let data_dir_output = data_dir_command.output().ok()?;
    let data_dir_stdout = canonical_output(&data_dir_output.stdout)?;
    let data_dir_stderr = canonical_output(&data_dir_output.stderr)?;
    if !data_dir_output.status.success() || !data_dir_stderr.trim().is_empty() {
        return None;
    }
    let installed_data_dir = canonical_reported_directory(&data_dir_stdout)?;
    let installed_primitive_root =
        canonical_directory(&installed_data_dir.join(AGDA_PRIMITIVE_RELATIVE_ROOT))?;

    let snapshot_data_dir = scratch.join(AGDA_DATA_SNAPSHOT_DIRECTORY_NAME);
    fs::create_dir(&snapshot_data_dir).ok()?;
    let snapshot_primitive_root = snapshot_data_dir.join(AGDA_PRIMITIVE_RELATIVE_ROOT);
    let installed_tree_before =
        materialize_primitive_tree(&installed_primitive_root, &snapshot_primitive_root)?;
    if installed_tree_before.digest.as_str() != AGDA_REFERENCE_PRIMITIVE_TREE_DIGEST {
        return None;
    }
    let snapshot_tree_before = collect_primitive_tree(&snapshot_primitive_root)?;
    if snapshot_tree_before != installed_tree_before {
        return None;
    }

    let source_path = scratch.join(AGDA_SOURCE_FILE_NAME);
    write_new_file(&source_path, AGDA_SOURCE_BYTES)?;
    let source_before = fs::read(&source_path).ok()?;
    if source_before != AGDA_SOURCE_BYTES {
        return None;
    }

    let mut checker_command = Command::new(executable);
    checker_command
        .args(AGDA_ARGUMENT_PROTOCOL)
        .arg("-i")
        .arg(scratch)
        .arg(&source_path);
    configure_sanitized_environment(
        &mut checker_command,
        scratch,
        Some(snapshot_data_dir.as_path()),
    );
    let checker_output = checker_command.output().ok()?;
    let checker_stdout = canonical_output(&checker_output.stdout)?;
    let checker_stderr = canonical_output(&checker_output.stderr)?;
    let expected_checker_line = format!(
        "Checking GscInductiveCoreV1 ({}).",
        source_path.to_string_lossy()
    );
    if !checker_output.status.success()
        || checker_stdout.trim() != expected_checker_line
        || !checker_stderr.trim().is_empty()
        || fs::read(&source_path).ok()? != source_before
    {
        return None;
    }

    let installed_tree_after = collect_primitive_tree(&installed_primitive_root)?;
    let snapshot_tree_after = collect_primitive_tree(&snapshot_primitive_root)?;
    if installed_tree_after != installed_tree_before || snapshot_tree_after != installed_tree_before
    {
        return None;
    }

    Some(AgdaGateEvidence {
        primitive_tree: installed_tree_before,
        data_dir_probe_argument_protocol_digest: data_dir_probe_argument_protocol_digest(),
        data_dir_probe_stdout_digest: Digest::of_bytes(data_dir_stdout.as_bytes()),
        data_dir_probe_stderr_digest: Digest::of_bytes(data_dir_stderr.as_bytes()),
        checker_stdout_digest: canonical_checker_stdout_digest(),
        checker_stderr_digest: Digest::of_bytes(checker_stderr.as_bytes()),
    })
}

pub fn replay_gsc_reference_vectors_v1(
    kernel: &Kernel,
    semantic: &VerifiedGscSemanticManifest,
) -> GscOutcome<VerifiedGscReferenceVectorReplay> {
    let expected_suite = &semantic.manifest().reference_vector_suite;
    if expected_suite.schema_version != 1 || expected_suite.ordered_cases.len() != 4 {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedManifest);
    }

    let two_nullary = match fixture(kernel, semantic, two_nullary_code(), b"two-nullary") {
        Ok(fixture) => fixture,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    let two_use = match require_proven(compile_use_v1(
        semantic,
        kernel,
        &two_nullary.signature,
        &two_nullary.frame,
    )) {
        Ok(family) => family,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    if two_use.ports().len() != 1 || two_nullary.code.constructor_ids().len() != 2 {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    }
    for constructor in two_nullary.code.constructor_ids() {
        if let Err(reason) = require_proven(compile_compute_v1(
            semantic,
            kernel,
            &two_nullary.signature,
            &two_nullary.frame,
            &two_use,
            two_use.ports()[0].key(),
            constructor,
        )) {
            return GscOutcome::Unknown(reason);
        }
    }

    let nonrecursive = match fixture(
        kernel,
        semantic,
        nonrecursive_argument_code(),
        b"nonrecursive",
    ) {
        Ok(fixture) => fixture,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    let nonrecursive_use = match require_proven(compile_use_v1(
        semantic,
        kernel,
        &nonrecursive.signature,
        &nonrecursive.frame,
    )) {
        Ok(family) => family,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    let nonrecursive_compute = match require_proven(compile_compute_v1(
        semantic,
        kernel,
        &nonrecursive.signature,
        &nonrecursive.frame,
        &nonrecursive_use,
        nonrecursive_use.ports()[0].key(),
        &nonrecursive.code.constructor_ids()[0],
    )) {
        Ok(family) => family,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    if nonrecursive_compute.output_clauses()[0].context().0.len() != 4 {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    }

    let recursive = match fixture(kernel, semantic, recursive_argument_code(), b"recursive") {
        Ok(fixture) => fixture,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    let recursive_use = match require_proven(compile_use_v1(
        semantic,
        kernel,
        &recursive.signature,
        &recursive.frame,
    )) {
        Ok(family) => family,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    let recursive_compute = match require_proven(compile_compute_v1(
        semantic,
        kernel,
        &recursive.signature,
        &recursive.frame,
        &recursive_use,
        recursive_use.ports()[0].key(),
        &recursive.code.constructor_ids()[0],
    )) {
        Ok(family) => family,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    let recursive_rhs_uses_exact_generated_call =
        match &recursive_compute.verification_judgments()[1] {
            OpenJudgment::HasType { term, .. } => matches!(
                term,
                Term::Apply {
                    argument,
                    ..
                } if matches!(
                    argument.as_ref(),
                    Term::Apply {
                        function,
                        argument,
                    } if function.as_ref() == &Term::Var { index: 1 }
                        && argument.as_ref() == &Term::Var { index: 0 }
                )
            ),
            _ => false,
        };
    if !recursive_rhs_uses_exact_generated_call {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    }

    let path = match fixture(kernel, semantic, path_mode_code(), b"path") {
        Ok(fixture) => fixture,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    if !matches!(
        compile_use_v1(semantic, kernel, &path.signature, &path.frame),
        GscOutcome::Unknown(GscUnknownReason::UnsupportedComputationMode)
    ) {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedComputationMode);
    }

    let ordered_dispositions = vec![
        GscReferenceVectorDisposition::ProvenSupported,
        GscReferenceVectorDisposition::ProvenSupported,
        GscReferenceVectorDisposition::ProvenGeneratedRecursiveCallBeta,
        GscReferenceVectorDisposition::UnknownUnsupportedComputationMode,
    ];
    if expected_suite
        .ordered_cases
        .iter()
        .map(|case| case.expected_disposition)
        .collect::<Vec<_>>()
        != ordered_dispositions
    {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedManifest);
    }
    let vector_suite_digest = expected_suite.canonical_digest();
    let digest = {
        let mut encoder = CanonicalEncoder::new();
        semantic.digest().encode_canonical(&mut encoder);
        vector_suite_digest.encode_canonical(&mut encoder);
        encoder.sequence(&ordered_dispositions);
        Digest::of_domain_bytes(
            "pen-demand/gsc-reference-vector-runtime-replay/v1",
            encoder.as_bytes(),
        )
    };
    GscOutcome::Proven(VerifiedGscReferenceVectorReplay {
        semantic_manifest_digest: semantic.digest().clone(),
        vector_suite_digest,
        ordered_dispositions,
        digest,
    })
}

struct Fixture {
    signature: VerifiedSignature,
    code: VerifiedClosedInductiveCode,
    frame: VerifiedClosedFormerFrame,
}

fn fixture(
    kernel: &Kernel,
    semantic: &VerifiedGscSemanticManifest,
    code: ClosedInductiveCode,
    namespace: &[u8],
) -> Result<Fixture, GscUnknownReason> {
    let owner = synthetic_global(namespace, b"owner");
    let introductions = (0..code.constructors.len())
        .map(|index| synthetic_global(namespace, format!("introduction-{index}").as_bytes()))
        .collect::<Vec<_>>();
    let mut declarations = vec![Declaration {
        id: owner.clone(),
        ty: Term::Sort { level: 0 },
        body: None,
    }];
    for (index, introduction) in introductions.iter().enumerate() {
        declarations.push(Declaration {
            id: introduction.clone(),
            ty: constructor_type(&code, &owner, index)?,
            body: None,
        });
    }
    let signature = kernel
        .verify_signature(&UncheckedSignature { declarations })
        .map_err(map_kernel_error)?;
    let code = require_proven(verify_closed_inductive_code(semantic, &code))?;
    let origin = GscOriginEventId(Digest::of_domain_chunks(
        "pen-demand/gsc-reference-origin/v1",
        &[namespace],
    ));
    let mut source_declarations = vec![owner.clone()];
    source_declarations.extend(introductions.iter().cloned());
    let frame = require_proven(verify_closed_former_frame(
        semantic,
        kernel,
        &signature,
        &code,
        &ClosedFormerFrame {
            schema_version: CLOSED_FORMER_FRAME_SCHEMA_VERSION,
            code_id: code.id().clone(),
            owner,
            introductions: code
                .constructor_ids()
                .iter()
                .cloned()
                .zip(introductions)
                .map(|(constructor, introduction)| IntroductionAlias {
                    constructor,
                    introduction,
                })
                .collect(),
            principal_sources: source_declarations
                .iter()
                .map(|declaration| PublicSourceId::for_declaration(&origin, declaration))
                .collect(),
            source_declarations,
            birth_support: vec![origin],
        },
    ))?;
    Ok(Fixture {
        signature,
        code,
        frame,
    })
}

fn synthetic_global(namespace: &[u8], label: &[u8]) -> GlobalId {
    GlobalId(Digest::of_domain_chunks(
        "pen-demand/gsc-reference-global/v1",
        &[namespace, label],
    ))
}

fn constructor_type(
    code: &ClosedInductiveCode,
    owner: &GlobalId,
    index: usize,
) -> Result<Term, GscUnknownReason> {
    let Some(constructor) = code.constructors.get(index) else {
        return Err(GscUnknownReason::MalformedCode);
    };
    let mut result = Term::Global { id: owner.clone() };
    for argument in constructor.arguments.0.iter().rev() {
        let parameter = match argument {
            CodeTerm::Sort { level } => Term::Sort { level: *level },
            CodeTerm::OwnerApp {
                parameters,
                indices,
            } if parameters.is_empty() && indices.is_empty() => Term::Global { id: owner.clone() },
            _ => return Err(GscUnknownReason::UnsupportedCode),
        };
        result = Term::Pi {
            parameter: Box::new(parameter),
            body: Box::new(result),
        };
    }
    Ok(result)
}

fn nullary_constructor() -> ConstructorCode {
    ConstructorCode {
        arguments: TelescopeCode::default(),
        result_indices: Vec::new(),
        recursive_positions: Vec::new(),
        boundary_ports: Vec::new(),
    }
}

fn two_nullary_code() -> ClosedInductiveCode {
    ClosedInductiveCode {
        schema_version: CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION,
        universe_level: 0,
        parameters: TelescopeCode::default(),
        indices: TelescopeCode::default(),
        constructors: vec![nullary_constructor(), nullary_constructor()],
        generated_computation_mode: ComputationMode::JudgmentalFreshHead,
    }
}

fn nonrecursive_argument_code() -> ClosedInductiveCode {
    ClosedInductiveCode {
        schema_version: CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION,
        universe_level: 0,
        parameters: TelescopeCode::default(),
        indices: TelescopeCode::default(),
        constructors: vec![ConstructorCode {
            arguments: TelescopeCode(vec![CodeTerm::Sort { level: 0 }]),
            result_indices: Vec::new(),
            recursive_positions: Vec::new(),
            boundary_ports: Vec::new(),
        }],
        generated_computation_mode: ComputationMode::JudgmentalFreshHead,
    }
}

fn recursive_argument_code() -> ClosedInductiveCode {
    ClosedInductiveCode {
        schema_version: CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION,
        universe_level: 0,
        parameters: TelescopeCode::default(),
        indices: TelescopeCode::default(),
        constructors: vec![ConstructorCode {
            arguments: TelescopeCode(vec![CodeTerm::OwnerApp {
                parameters: Vec::new(),
                indices: Vec::new(),
            }]),
            result_indices: Vec::new(),
            recursive_positions: vec![0],
            boundary_ports: Vec::new(),
        }],
        generated_computation_mode: ComputationMode::JudgmentalFreshHead,
    }
}

fn path_mode_code() -> ClosedInductiveCode {
    ClosedInductiveCode {
        schema_version: CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION,
        universe_level: 0,
        parameters: TelescopeCode::default(),
        indices: TelescopeCode::default(),
        constructors: vec![nullary_constructor()],
        generated_computation_mode: ComputationMode::PathTerm,
    }
}

fn require_proven<T>(outcome: GscOutcome<T>) -> Result<T, GscUnknownReason> {
    match outcome {
        GscOutcome::Proven(value) => Ok(value),
        GscOutcome::Unknown(reason) => Err(reason),
    }
}

fn map_kernel_error(error: KernelError) -> GscUnknownReason {
    match error {
        KernelError::ResourceExhausted(_) => GscUnknownReason::ResourceExhausted,
        _ => GscUnknownReason::KernelCouldNotCertify,
    }
}

fn materialize_primitive_tree(
    installed_root: &Path,
    snapshot_root: &Path,
) -> Option<PrimitiveSourceTreeEvidence> {
    if let Some(parent) = snapshot_root.parent() {
        fs::create_dir_all(parent).ok()?;
    }
    fs::create_dir(snapshot_root).ok()?;
    collect_primitive_tree_inner(installed_root, Some(snapshot_root))
}

fn collect_primitive_tree(root: &Path) -> Option<PrimitiveSourceTreeEvidence> {
    collect_primitive_tree_inner(root, None)
}

fn collect_primitive_tree_inner(
    root: &Path,
    snapshot_root: Option<&Path>,
) -> Option<PrimitiveSourceTreeEvidence> {
    let mut pending = vec![root.to_owned()];
    let mut visited_entries = 0_u64;
    let mut source_bytes = 0_u64;
    let mut files = Vec::new();

    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).ok()? {
            let entry = entry.ok()?;
            visited_entries = visited_entries.checked_add(1)?;
            if visited_entries > MAX_AGDA_PRIMITIVE_TREE_ENTRIES {
                return None;
            }
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path).ok()?;
            let file_type = metadata.file_type();
            if file_type.is_symlink() {
                return None;
            }
            if file_type.is_dir() {
                pending.push(path);
                continue;
            }
            if !file_type.is_file() {
                return None;
            }

            let relative = path.strip_prefix(root).ok()?;
            let relative_path = relative.to_str()?.replace('\\', "/");
            if !is_primitive_source_file(&relative_path) {
                continue;
            }
            if files.len() as u64 >= MAX_AGDA_PRIMITIVE_SOURCE_FILES
                || metadata.len() > MAX_AGDA_PRIMITIVE_SOURCE_FILE_BYTES
            {
                return None;
            }
            let raw_bytes = fs::read(&path).ok()?;
            if raw_bytes.len() as u64 > MAX_AGDA_PRIMITIVE_SOURCE_FILE_BYTES {
                return None;
            }
            let bytes = canonical_primitive_source_bytes(&relative_path, &raw_bytes)?;
            source_bytes = source_bytes.checked_add(bytes.len() as u64)?;
            if source_bytes > MAX_AGDA_PRIMITIVE_SOURCE_TREE_BYTES {
                return None;
            }
            if let Some(snapshot_root) = snapshot_root {
                let snapshot_path = snapshot_root.join(relative);
                if let Some(parent) = snapshot_path.parent() {
                    fs::create_dir_all(parent).ok()?;
                }
                write_new_file(&snapshot_path, &bytes)?;
            }
            files.push(PrimitiveSourceFileEvidence {
                relative_path,
                byte_length: bytes.len() as u64,
                digest: Digest::of_bytes(&bytes),
            });
        }
    }

    files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    if files.is_empty()
        || files
            .windows(2)
            .any(|pair| pair[0].relative_path >= pair[1].relative_path)
    {
        return None;
    }
    let digest = primitive_tree_digest(&files);
    Some(PrimitiveSourceTreeEvidence {
        digest,
        source_file_count: files.len() as u64,
        source_byte_length: source_bytes,
        files,
    })
}

fn is_primitive_source_file(relative_path: &str) -> bool {
    relative_path == "agda-builtins.agda-lib"
        || relative_path.ends_with(".agda")
        || relative_path.ends_with(".lagda")
        || relative_path.ends_with(".lagda.md")
        || relative_path.ends_with(".lagda.rst")
        || relative_path.ends_with(".lagda.tex")
        || relative_path.ends_with(".agdai")
}

fn canonical_primitive_source_bytes(relative_path: &str, raw: &[u8]) -> Option<Vec<u8>> {
    if relative_path.ends_with(".agdai") {
        return Some(raw.to_vec());
    }
    let text = std::str::from_utf8(raw).ok()?;
    let normalized = text.replace("\r\n", "\n");
    (!normalized.contains('\r')).then(|| normalized.into_bytes())
}

fn primitive_tree_digest(files: &[PrimitiveSourceFileEvidence]) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.text(AGDA_PRIMITIVE_SOURCE_TREE_CONTRACT_ID);
    encoder.u64(files.len() as u64);
    for file in files {
        encoder.text(&file.relative_path);
        encoder.u64(file.byte_length);
        encoder.text(file.digest.as_str());
    }
    Digest::of_bytes(encoder.as_bytes())
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

fn canonical_reported_directory(stdout: &str) -> Option<PathBuf> {
    let lines = stdout.lines().collect::<Vec<_>>();
    let [reported] = lines.as_slice() else {
        return None;
    };
    let reported = reported.trim();
    if reported.is_empty() || !Path::new(reported).is_absolute() {
        return None;
    }
    canonical_directory(Path::new(reported))
}

fn canonical_directory(path: &Path) -> Option<PathBuf> {
    let path = fs::canonicalize(path).ok()?;
    path.is_dir().then_some(path)
}

fn configure_sanitized_environment(
    command: &mut Command,
    scratch: &Path,
    agda_data_dir: Option<&Path>,
) {
    command.env_clear();
    for key in ["SystemRoot", "WINDIR", "ComSpec", "PATHEXT"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    command
        .current_dir(scratch)
        .env("HOME", scratch)
        .env("USERPROFILE", scratch)
        .env("XDG_CONFIG_HOME", scratch)
        .env("APPDATA", scratch)
        .env("LOCALAPPDATA", scratch)
        .env("TMP", scratch)
        .env("TEMP", scratch)
        .env("TMPDIR", scratch)
        .env("AGDA_DIR", scratch.join("agda-app"))
        .env("LC_ALL", "C.UTF-8")
        .env("LANG", "C.UTF-8");
    if let Some(agda_data_dir) = agda_data_dir {
        command.env("Agda_datadir", agda_data_dir);
    }
}

fn data_dir_probe_argument_protocol_digest() -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.u16(1);
    encoder.text("--print-agda-data-dir");
    encode_sanitized_environment_protocol(&mut encoder, false);
    Digest::of_domain_bytes(
        "pen-demand/gsc-agda-data-dir-probe-argument-protocol/v1",
        encoder.as_bytes(),
    )
}

fn checker_argument_protocol_digest() -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.u16(2);
    encoder.u64(AGDA_ARGUMENT_PROTOCOL.len() as u64);
    for argument in AGDA_ARGUMENT_PROTOCOL {
        encoder.text(argument);
    }
    encoder.text("-i");
    encoder.text("<private-scratch>");
    encoder.text(AGDA_SOURCE_FILE_NAME);
    encoder.text("current-directory=<private-scratch>");
    encoder.text("AGDA_DIR=<private-scratch>/agda-app");
    encoder.text("Agda_datadir=<private-scratch>/agda-data");
    encode_sanitized_environment_protocol(&mut encoder, true);
    Digest::of_domain_bytes(
        "pen-demand/gsc-agda-checker-argument-protocol/v1",
        encoder.as_bytes(),
    )
}

fn encode_sanitized_environment_protocol(
    encoder: &mut CanonicalEncoder,
    includes_agda_data_dir: bool,
) {
    encoder.text("environment=cleared");
    encoder.text("preserve-if-present=SystemRoot,WINDIR,ComSpec,PATHEXT");
    encoder.text(
        "private=HOME,USERPROFILE,XDG_CONFIG_HOME,APPDATA,LOCALAPPDATA,TMP,TEMP,TMPDIR,AGDA_DIR",
    );
    encoder.text("locale=LC_ALL:C.UTF-8,LANG:C.UTF-8");
    encoder.text(if includes_agda_data_dir {
        "Agda_datadir=private-snapshot"
    } else {
        "Agda_datadir=unset"
    });
}

fn canonical_checker_stdout_digest() -> Digest {
    Digest::of_bytes(b"Checking GscInductiveCoreV1 (<private-source>).\n")
}

fn reference_agreement_digest(agreement: &VerifiedGscReferenceAgreement) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    agreement
        .semantic_manifest_digest
        .encode_canonical(&mut encoder);
    agreement.vector_suite_digest.encode_canonical(&mut encoder);
    agreement.rust_replay_digest.encode_canonical(&mut encoder);
    agreement.agda_source_digest.encode_canonical(&mut encoder);
    agreement
        .agda_executable_digest
        .encode_canonical(&mut encoder);
    encoder.text(&agreement.agda_version);
    agreement
        .agda_primitive_tree_digest
        .encode_canonical(&mut encoder);
    encoder.u64(agreement.agda_primitive_source_file_count);
    encoder.u64(agreement.agda_primitive_source_byte_length);
    agreement
        .agda_data_dir_probe_argument_protocol_digest
        .encode_canonical(&mut encoder);
    agreement
        .agda_data_dir_probe_stdout_digest
        .encode_canonical(&mut encoder);
    agreement
        .agda_data_dir_probe_stderr_digest
        .encode_canonical(&mut encoder);
    agreement
        .checker_argument_protocol_digest
        .encode_canonical(&mut encoder);
    agreement
        .checker_stdout_digest
        .encode_canonical(&mut encoder);
    agreement
        .checker_stderr_digest
        .encode_canonical(&mut encoder);
    agreement
        .gate_protocol_digest
        .encode_canonical(&mut encoder);
    Digest::of_domain_bytes(
        "pen-demand/gsc-reference-agreement-capability/v1",
        encoder.as_bytes(),
    )
}

fn resolve_agda_executable() -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    let extensions = executable_extensions();
    for directory in std::env::split_paths(&path) {
        for extension in &extensions {
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
            .filter(|extension| !extension.is_empty())
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

fn parse_agda_version(output: &str) -> Option<&str> {
    output.lines().next()?.trim().strip_prefix("Agda version ")
}

fn canonical_output(bytes: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(bytes).ok()?;
    let normalized = text.replace("\r\n", "\n");
    (!normalized.contains('\r')).then_some(normalized)
}

fn create_scratch_directory() -> Option<PathBuf> {
    let base = std::env::temp_dir();
    for _ in 0..128 {
        let ordinal = SCRATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = base.join(format!(
            "pen-demand-gsc-reference-{}-{ordinal}",
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

#[cfg(test)]
mod primitive_tree_tests {
    use super::{
        PrimitiveSourceFileEvidence, canonical_primitive_source_bytes, primitive_tree_digest,
    };
    use pen_kernel::Digest;

    fn file(path: &str, bytes: &[u8]) -> PrimitiveSourceFileEvidence {
        PrimitiveSourceFileEvidence {
            relative_path: path.to_owned(),
            byte_length: bytes.len() as u64,
            digest: Digest::of_bytes(bytes),
        }
    }

    #[test]
    fn primitive_tree_digest_binds_paths_lengths_order_and_bytes() {
        let first = vec![
            file("Agda/Builtin/Equality.agda", b"equality\n"),
            file("Agda/Builtin/Nat.agda", b"nat\n"),
        ];
        let mut changed_bytes = first.clone();
        changed_bytes[1] = file("Agda/Builtin/Nat.agda", b"changed\n");
        let mut changed_order = first.clone();
        changed_order.reverse();
        let mut changed_path = first.clone();
        changed_path[1].relative_path = "Agda/Builtin/String.agda".to_owned();

        assert_ne!(
            primitive_tree_digest(&first),
            primitive_tree_digest(&changed_bytes)
        );
        assert_ne!(
            primitive_tree_digest(&first),
            primitive_tree_digest(&changed_order)
        );
        assert_ne!(
            primitive_tree_digest(&first),
            primitive_tree_digest(&changed_path)
        );
    }

    #[test]
    fn primitive_text_is_lf_canonical_but_interfaces_are_raw() {
        assert_eq!(
            canonical_primitive_source_bytes("Agda/Builtin/Nat.agda", b"a\r\nb\r\n"),
            Some(b"a\nb\n".to_vec())
        );
        assert!(canonical_primitive_source_bytes("Agda/Builtin/Nat.agda", b"a\rb").is_none());
        assert_eq!(
            canonical_primitive_source_bytes("Agda/Builtin/Nat.agdai", b"\x00\r\xff"),
            Some(b"\x00\r\xff".to_vec())
        );
    }
}
