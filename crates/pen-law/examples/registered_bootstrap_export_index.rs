use pen_demand::gsc::{
    CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION, ClosedInductiveCode, ComputationMode, ConstructorCode,
    GscOutcome, TelescopeCode, VerifiedGscSemanticManifest, frozen_gsc_semantic_manifest_v1,
    verify_gsc_semantic_manifest_v1,
};
use pen_kernel::{Digest, Kernel, KernelLimits, UncheckedSignature};
use pen_law::{
    REGISTERED_BOOTSTRAP_EXPORT_CODEC_VERSION, REGISTERED_BOOTSTRAP_EXPORT_SCHEMA_VERSION,
    RegisteredInductiveErasureRuleV1, RegisteredIntroductionAliasV1,
    UncheckedInductiveAliasErasureV1, UncheckedRegisteredBootstrapExportIndexV1,
    UncheckedRegisteredGroupDispositionV1, VerifiedRegisteredBootstrap,
    load_embedded_registered_bootstrap, verify_registered_bootstrap_export_index_bytes_v1,
};
use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let [command, path] = arguments.as_slice() else {
        return Err("usage: registered_bootstrap_export_index <create-new|replay> <path>".into());
    };

    let kernel = Kernel::new(KernelLimits::default())?;
    let bootstrap = load_embedded_registered_bootstrap(&kernel)?;
    let semantic_manifest = verified_manifest()?;
    match command.as_str() {
        "create-new" => {
            let disclosure = fresh_disclosure(&bootstrap, &semantic_manifest);
            let mut bytes = serde_json::to_vec_pretty(&disclosure)?;
            bytes.push(b'\n');
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &semantic_manifest,
                &bytes,
            )?;
            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(Path::new(path))?;
            file.write_all(&bytes)?;
            println!("{}", disclosure.claimed_subject_digest);
        }
        "replay" => {
            let bytes = fs::read(Path::new(path))?;
            let verified = verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &semantic_manifest,
                &bytes,
            )?;
            println!("{}", verified.digest());
        }
        _ => return Err("command must be create-new or replay".into()),
    }
    Ok(())
}

fn verified_manifest() -> Result<VerifiedGscSemanticManifest, Box<dyn Error>> {
    match verify_gsc_semantic_manifest_v1(&frozen_gsc_semantic_manifest_v1()) {
        GscOutcome::Proven(manifest) => Ok(manifest),
        GscOutcome::Unknown(reason) => {
            Err(format!("frozen semantic manifest is unknown: {reason:?}").into())
        }
    }
}

fn fresh_disclosure(
    bootstrap: &VerifiedRegisteredBootstrap,
    semantic_manifest: &VerifiedGscSemanticManifest,
) -> UncheckedRegisteredBootstrapExportIndexV1 {
    let declarations = bootstrap.final_boundary().declarations();
    let primitive_code = ClosedInductiveCode {
        schema_version: CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION,
        universe_level: 0,
        parameters: TelescopeCode::default(),
        indices: TelescopeCode::default(),
        constructors: vec![ConstructorCode {
            arguments: TelescopeCode::default(),
            result_indices: Vec::new(),
            recursive_positions: Vec::new(),
            boundary_ports: Vec::new(),
        }],
        generated_computation_mode: ComputationMode::JudgmentalFreshHead,
    };
    let mut disclosure = UncheckedRegisteredBootstrapExportIndexV1 {
        schema_version: REGISTERED_BOOTSTRAP_EXPORT_SCHEMA_VERSION,
        codec_version: REGISTERED_BOOTSTRAP_EXPORT_CODEC_VERSION,
        semantic_manifest_digest: semantic_manifest.digest().clone(),
        bootstrap_contract_digest: bootstrap.bootstrap_contract_digest().clone(),
        bootstrap_artifact_digest: bootstrap.artifact_digest().clone(),
        final_checked_boundary_digest: bootstrap.final_boundary().digest().clone(),
        boundary_chain: bootstrap.boundary_chain().to_vec(),
        registered_source_identities: bootstrap.source_identities().to_vec(),
        registered_binding_identities: bootstrap.binding_identities().to_vec(),
        group_dispositions: vec![
            UncheckedRegisteredGroupDispositionV1::PlainPublicDeclaration {
                declaration: declarations[0].id.clone(),
            },
            UncheckedRegisteredGroupDispositionV1::InductiveAlias {
                owner: declarations[1].id.clone(),
                primitive_code,
                introduction_aliases: vec![RegisteredIntroductionAliasV1 {
                    constructor_ordinal: 0,
                    declaration: declarations[2].id.clone(),
                }],
                erasure: UncheckedInductiveAliasErasureV1 {
                    rule: RegisteredInductiveErasureRuleV1::NativeClosedSingleton,
                    source_identities: bootstrap.source_identities()[1..].to_vec(),
                    normalized_declarations: UncheckedSignature {
                        declarations: declarations[1..].to_vec(),
                    },
                },
            },
        ],
        registered_q3_theorems: Vec::new(),
        claimed_subject_digest: Digest::of_bytes(b"replaced below"),
    };
    disclosure.claimed_subject_digest = disclosure.subject_digest();
    disclosure
}
