use crate::manifest::{AgdaExportManifest, AgdaExportedStep, VerificationStatus};
use crate::render::{render_step_module, step_module_name, step_source_file, step_verify_log_file};
use crate::verify::verify_manifest;
use anyhow::{Context, Result};
use pen_core::telescope::Telescope;
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
    pub telescope: Telescope,
}

pub fn export_steps(
    output_dir: &Path,
    run_id: &str,
    steps: &[ExportStepInput],
    verify_requested: bool,
) -> Result<AgdaExportManifest> {
    fs::create_dir_all(output_dir).with_context(|| format!("create {}", output_dir.display()))?;

    for step in steps {
        let source = render_step_module(
            step.step_index,
            &step.label,
            &step.telescope,
            &step.candidate_hash,
            &step.canonical_hash,
        );
        let source_path = output_dir.join(step_source_file(step.step_index));
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
        steps: steps
            .iter()
            .map(|step| AgdaExportedStep {
                step_index: step.step_index,
                label: step.label.clone(),
                module_name: step_module_name(step.step_index),
                source_file: step_source_file(step.step_index),
                verify_log_file: step_verify_log_file(step.step_index),
                candidate_hash: step.candidate_hash.clone(),
                canonical_hash: step.canonical_hash.clone(),
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

#[cfg(test)]
mod tests {
    use super::{ExportStepInput, export_steps};
    use crate::manifest::VerificationStatus;
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
                candidate_hash: "blake3:candidate".to_owned(),
                canonical_hash: "blake3:canonical".to_owned(),
                telescope: Telescope::reference(1),
            }],
            false,
        )
        .expect("export should succeed");

        assert_eq!(manifest.steps[0].verification, VerificationStatus::Pending);
        assert!(output_dir.join("Step01.agda").exists());
        assert!(output_dir.join("manifest.json").exists());

        fs::remove_dir_all(output_dir).ok();
    }
}
