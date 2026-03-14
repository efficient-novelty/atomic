use crate::manifest::{AgdaExportManifest, VerificationStatus};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn verify_manifest(output_dir: &Path, manifest: &mut AgdaExportManifest) -> Result<()> {
    let agda_available = Command::new("agda").arg("--version").output().is_ok();

    for step in &mut manifest.steps {
        let log_path = output_dir.join(&step.verify_log_file);
        if !agda_available {
            fs::write(&log_path, "skipped: agda executable not found\n")
                .with_context(|| format!("write {}", log_path.display()))?;
            step.verification = VerificationStatus::Skipped;
            continue;
        }

        let source_path = output_dir.join(&step.source_file);
        let output = Command::new("agda")
            .arg(format!("-i{}", output_dir.display()))
            .arg(&source_path)
            .output()
            .with_context(|| format!("run agda for {}", source_path.display()))?;

        let mut log = String::new();
        log.push_str(&String::from_utf8_lossy(&output.stdout));
        log.push_str(&String::from_utf8_lossy(&output.stderr));
        fs::write(&log_path, log).with_context(|| format!("write {}", log_path.display()))?;

        step.verification = if output.status.success() {
            VerificationStatus::Passed
        } else {
            VerificationStatus::Failed
        };
    }

    Ok(())
}
