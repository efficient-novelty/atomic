use crate::manifest::{AgdaExportManifest, VerificationStatus};
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn verify_manifest(output_dir: &Path, manifest: &mut AgdaExportManifest) -> Result<()> {
    let agda_available = Command::new("agda").arg("--version").output().is_ok();

    for step in &mut manifest.steps {
        let log_path = output_dir.join(&step.verify_log_file);
        let mut log = match verify_step_contract(output_dir, step) {
            Ok(log) => log,
            Err(error) => {
                let log = format!("contract failed: {error:#}\n");
                fs::write(&log_path, &log)
                    .with_context(|| format!("write {}", log_path.display()))?;
                step.verification = VerificationStatus::Failed;
                continue;
            }
        };

        if !agda_available {
            log.push_str("skipped: agda executable not found\n");
            fs::write(&log_path, log).with_context(|| format!("write {}", log_path.display()))?;
            step.verification = VerificationStatus::Skipped;
            continue;
        }

        let source_path = output_dir.join(&step.source_file);
        let output = Command::new("agda")
            .arg(format!("-i{}", output_dir.display()))
            .arg(&source_path)
            .output()
            .with_context(|| format!("run agda for {}", source_path.display()))?;

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

fn verify_step_contract(
    output_dir: &Path,
    step: &crate::manifest::AgdaExportedStep,
) -> Result<String> {
    let source_path = output_dir.join(&step.source_file);
    let payload_path = output_dir.join(&step.payload_file);
    let source = fs::read_to_string(&source_path)
        .with_context(|| format!("read {}", source_path.display()))?;
    let payload = fs::read_to_string(&payload_path)
        .with_context(|| format!("read {}", payload_path.display()))?;

    require_contains(
        &source,
        &format!("module {} where", step.module_name),
        "step module declaration",
    )?;
    require_contains(
        &source,
        &format!(
            "open import {} as {}",
            step.payload_module_name, step.payload_module_name
        ),
        "payload import hook",
    )?;
    require_contains(
        &source,
        &format!("-- candidate_hash: {}", step.candidate_hash),
        "step candidate hash comment",
    )?;
    require_contains(
        &source,
        &format!("-- canonical_hash: {}", step.canonical_hash),
        "step canonical hash comment",
    )?;

    for import_step in &step.claims.import_steps {
        let module_name = format!("Step{import_step:02}");
        require_contains(
            &source,
            &format!("open import {module_name} as {module_name}"),
            "prior step import",
        )?;
    }

    require_contains(
        &payload,
        &format!("module {} where", step.payload_module_name),
        "payload module declaration",
    )?;
    require_contains(
        &payload,
        &format!("-- candidate_hash: {}", step.candidate_hash),
        "payload candidate hash comment",
    )?;
    require_contains(
        &payload,
        &format!("-- canonical_hash: {}", step.canonical_hash),
        "payload canonical hash comment",
    )?;
    require_contains(
        &payload,
        &format!("-- canonical_key: {}", step.claims.canonical_key),
        "payload canonical key comment",
    )?;
    require_contains(
        &payload,
        &format!("-- bit_kappa: {}", step.claims.bit_kappa),
        "payload bit kappa comment",
    )?;
    require_contains(
        &payload,
        &format!("-- clause_kappa: {}", step.claims.clause_kappa),
        "payload clause kappa comment",
    )?;
    require_contains(
        &payload,
        &format!("-- desugared_kappa: {}", step.claims.desugared_kappa),
        "payload desugared kappa comment",
    )?;
    require_contains(
        &payload,
        &format!("-- rho: {}", step.claims.rho),
        "payload rho comment",
    )?;
    require_contains(
        &payload,
        &format!(
            "-- import_steps: {}",
            render_import_steps(&step.claims.import_steps)
        ),
        "payload import step comment",
    )?;
    require_contains(
        &payload,
        &format!("-- nu_g: {}", step.claims.nu_claim.nu_g),
        "payload nu_g comment",
    )?;
    require_contains(
        &payload,
        &format!("-- nu_h: {}", step.claims.nu_claim.nu_h),
        "payload nu_h comment",
    )?;
    require_contains(
        &payload,
        &format!("-- nu_c: {}", step.claims.nu_claim.nu_c),
        "payload nu_c comment",
    )?;
    require_contains(
        &payload,
        &format!("-- nu_total: {}", step.claims.nu_claim.nu_total),
        "payload nu_total comment",
    )?;

    let summed = step.claims.nu_claim.nu_g + step.claims.nu_claim.nu_h + step.claims.nu_claim.nu_c;
    if summed != step.claims.nu_claim.nu_total {
        bail!(
            "payload nu claim is inconsistent for step {}: {} + {} + {} != {}",
            step.step_index,
            step.claims.nu_claim.nu_g,
            step.claims.nu_claim.nu_h,
            step.claims.nu_claim.nu_c,
            step.claims.nu_claim.nu_total
        );
    }

    Ok(format!(
        "contract: payload and witness checks passed for step {}\n",
        step.step_index
    ))
}

fn require_contains(haystack: &str, needle: &str, description: &str) -> Result<()> {
    if haystack.contains(needle) {
        Ok(())
    } else {
        bail!("missing {description}: expected `{needle}`")
    }
}

fn render_import_steps(import_steps: &[u32]) -> String {
    if import_steps.is_empty() {
        "none".to_owned()
    } else {
        import_steps
            .iter()
            .map(|step| format!("{step:02}"))
            .collect::<Vec<_>>()
            .join(",")
    }
}
