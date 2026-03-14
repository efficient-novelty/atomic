use crate::manifest::{
    AgdaExportManifest, AgdaExportedStep, ExportSource, NuClaim, ProofClaims, VerificationStatus,
};
use crate::render::{
    payload_module_name, payload_source_file, render_payload_module, render_step_module,
    step_module_name, step_source_file, step_verify_log_file,
};
use crate::verify::verify_manifest;
use anyhow::{Context, Result, bail};
use pen_core::canonical::canonical_key_telescope;
use pen_core::encode::telescope_bit_cost;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use pen_eval::bar::compute_rho;
use pen_eval::nu::structural_nu;
use std::fs;
use std::path::Path;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportStepInput {
    pub step_index: u32,
    pub label: String,
    pub candidate_hash: String,
    pub canonical_hash: String,
    pub bit_kappa: u16,
    pub clause_kappa: u16,
    pub nu: u16,
    pub rho: Rational,
    pub telescope: Telescope,
}

pub fn export_steps(
    output_dir: &Path,
    run_id: &str,
    steps: &[ExportStepInput],
    verify_requested: bool,
    source: ExportSource,
) -> Result<AgdaExportManifest> {
    fs::create_dir_all(output_dir).with_context(|| format!("create {}", output_dir.display()))?;

    let prepared_steps = prepare_steps(steps)?;
    for step in &prepared_steps {
        let payload_source = render_payload_module(
            step.input.step_index,
            &step.input.label,
            &step.input.candidate_hash,
            &step.input.canonical_hash,
            &step.claims,
        );
        let payload_path = output_dir.join(payload_source_file(step.input.step_index));
        fs::write(&payload_path, payload_source)
            .with_context(|| format!("write {}", payload_path.display()))?;

        let source = render_step_module(
            step.input.step_index,
            &step.input.label,
            &step.input.telescope,
            &step.input.candidate_hash,
            &step.input.canonical_hash,
            &payload_module_name(step.input.step_index),
        );
        let source_path = output_dir.join(step_source_file(step.input.step_index));
        fs::write(&source_path, source)
            .with_context(|| format!("write {}", source_path.display()))?;
    }

    let generated_utc = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .context("format export timestamp")?;
    let mut manifest = AgdaExportManifest {
        schema_version: 1,
        run_id: run_id.to_owned(),
        generated_utc,
        output_dir: output_dir.display().to_string(),
        verify_requested,
        source,
        steps: prepared_steps
            .iter()
            .map(|step| AgdaExportedStep {
                step_index: step.input.step_index,
                label: step.input.label.clone(),
                module_name: step_module_name(step.input.step_index),
                source_file: step_source_file(step.input.step_index),
                payload_module_name: payload_module_name(step.input.step_index),
                payload_file: payload_source_file(step.input.step_index),
                verify_log_file: step_verify_log_file(step.input.step_index),
                candidate_hash: step.input.candidate_hash.clone(),
                canonical_hash: step.input.canonical_hash.clone(),
                claims: step.claims.clone(),
                verification: VerificationStatus::Pending,
            })
            .collect(),
    };

    if verify_requested {
        verify_manifest(output_dir, &mut manifest)?;
    }

    let manifest_path = output_dir.join("manifest.json");
    let json = serde_json::to_string_pretty(&manifest).context("serialize export manifest")?;
    fs::write(&manifest_path, format!("{json}\n"))
        .with_context(|| format!("write {}", manifest_path.display()))?;

    Ok(manifest)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PreparedStep {
    input: ExportStepInput,
    claims: ProofClaims,
}

fn prepare_steps(steps: &[ExportStepInput]) -> Result<Vec<PreparedStep>> {
    let mut library: Library = Vec::new();
    let mut nu_history = Vec::new();
    let mut prepared = Vec::with_capacity(steps.len());

    for (offset, step) in steps.iter().enumerate() {
        let expected_step = u32::try_from(offset + 1).expect("step count exceeded u32");
        if step.step_index != expected_step {
            bail!(
                "Agda export expects a contiguous 1-based prefix, found step {} at position {}",
                step.step_index,
                expected_step
            );
        }

        let candidate_json = serde_json::to_vec(&step.telescope)
            .context("serialize telescope for candidate hash")?;
        let candidate_hash = format!("blake3:{}", blake3_hex(&candidate_json));
        if candidate_hash != step.candidate_hash {
            bail!(
                "step {} candidate hash mismatch: accepted {} but telescope encoded {}",
                step.step_index,
                step.candidate_hash,
                candidate_hash
            );
        }

        let canonical_key = canonical_key_telescope(&step.telescope);
        let canonical_hash = format!("blake3:{}", blake3_hex(canonical_key.0.as_bytes()));
        if canonical_hash != step.canonical_hash {
            bail!(
                "step {} canonical hash mismatch: accepted {} but canonical key encoded {}",
                step.step_index,
                step.canonical_hash,
                canonical_hash
            );
        }

        let bit_kappa =
            u16::try_from(telescope_bit_cost(&step.telescope)).expect("bit kappa exceeded u16");
        if bit_kappa != step.bit_kappa {
            bail!(
                "step {} bit kappa mismatch: accepted {} but telescope encoded {}",
                step.step_index,
                step.bit_kappa,
                bit_kappa
            );
        }

        let clause_kappa =
            u16::try_from(step.telescope.kappa()).expect("clause kappa exceeded u16");
        if clause_kappa != step.clause_kappa {
            bail!(
                "step {} clause kappa mismatch: accepted {} but telescope encoded {}",
                step.step_index,
                step.clause_kappa,
                clause_kappa
            );
        }

        let rho = compute_rho(u32::from(step.nu), u32::from(step.clause_kappa))
            .expect("rho should exist for non-zero clause kappa");
        if rho != step.rho {
            bail!(
                "step {} rho mismatch: accepted {} but nu/kappa encoded {}",
                step.step_index,
                step.rho,
                rho
            );
        }

        let nu_result = structural_nu(&step.telescope, &library, &nu_history);
        if nu_result.total != u32::from(step.nu) {
            bail!(
                "step {} nu mismatch: accepted {} but structural export encoded {}",
                step.step_index,
                step.nu,
                nu_result.total
            );
        }

        let desugared_kappa =
            u16::try_from(step.telescope.desugared_kappa()).expect("desugared kappa exceeded u16");
        let claims = ProofClaims {
            canonical_key: canonical_key.0,
            bit_kappa,
            clause_kappa,
            desugared_kappa,
            rho,
            import_steps: step.telescope.lib_refs().into_iter().collect(),
            nu_claim: NuClaim {
                nu_g: nu_result.nu_g,
                nu_h: nu_result.nu_h,
                nu_c: nu_result.nu_c,
                nu_total: nu_result.total,
            },
        };

        prepared.push(PreparedStep {
            input: step.clone(),
            claims,
        });

        nu_history.push((step.step_index, nu_result.total));
        library.push(LibraryEntry::from_telescope(&step.telescope, &library));
    }

    Ok(prepared)
}

#[cfg(test)]
mod tests {
    use super::{ExportStepInput, export_steps};
    use crate::manifest::{ExportSource, VerificationStatus};
    use pen_core::rational::Rational;
    use pen_core::telescope::Telescope;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("pen-agda-{name}-{id}"));
        fs::create_dir_all(&dir).expect("temp dir should be created");
        dir
    }

    #[test]
    fn export_writes_manifest_and_modules() {
        let output_dir = temp_dir("export");
        let manifest = export_steps(
            &output_dir,
            "run-1",
            &[ExportStepInput {
                step_index: 1,
                label: "Universe".to_owned(),
                candidate_hash:
                    "blake3:61631d63a5877aad1b32b1e71aa6f0e555317b65732f327e3f38e32c222e29e7"
                        .to_owned(),
                canonical_hash:
                    "blake3:23ab301fcb806f4f151974b767119ea66f8ee39bcf5b7d9c95924fc2902663b2"
                        .to_owned(),
                bit_kappa: 14,
                clause_kappa: 2,
                nu: 1,
                rho: Rational::new(1, 2),
                telescope: Telescope::reference(1),
            }],
            false,
            ExportSource::ReferenceReplayFallback,
        )
        .expect("export should succeed");

        assert_eq!(manifest.steps[0].verification, VerificationStatus::Pending);
        assert!(output_dir.join("Step01.agda").exists());
        assert!(output_dir.join("Payload01.agda").exists());
        assert!(output_dir.join("manifest.json").exists());
        assert_eq!(manifest.source, ExportSource::ReferenceReplayFallback);
        assert_eq!(manifest.steps[0].claims.nu_claim.nu_total, 1);

        fs::remove_dir_all(output_dir).ok();
    }
}
