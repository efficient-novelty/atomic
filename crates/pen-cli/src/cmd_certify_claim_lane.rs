use crate::claim_evidence::{
    EARLY_GENERATED_TARGETS, LATE_GENERATED_FLOORS, ManifestCompletenessCheck,
    RuntimeThresholdCheck, accepted_hash_parity_check, breadth_check, build_claim_lane_audit,
    current_utc_timestamp, load_run, manifest_completeness_check, runtime_threshold_check,
    write_text,
};
use crate::cli::CertifyClaimLaneArgs;
use anyhow::{Result, bail};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
struct SimpleCheck {
    status: String,
    detail: String,
}

#[derive(Clone, Debug, Serialize)]
struct CoverageCheck {
    status: String,
    detail: String,
}

#[derive(Clone, Debug, Serialize)]
struct ClaimCertificate {
    certificate_version: u32,
    generated_utc: String,
    status: String,
    claim_run: String,
    guarded_run: String,
    runtime_threshold_ms: Option<u64>,
    failing_checks: Vec<String>,
    checks: ClaimCertificateChecks,
}

#[derive(Clone, Debug, Serialize)]
struct ClaimCertificateChecks {
    accepted_hash_parity: SimpleCheck,
    search_policy: SimpleCheck,
    fallback_honesty: SimpleCheck,
    narrative_artifacts: SimpleCheck,
    early_breadth: crate::claim_evidence::BreadthCheck,
    late_generated_floors: crate::claim_evidence::BreadthCheck,
    runtime_threshold: RuntimeThresholdCheck,
    exact_screen_reason_completeness: CoverageCheck,
    prune_class_completeness: CoverageCheck,
    manifest_completeness: ManifestCompletenessCheck,
}

pub fn certify_claim_lane(args: CertifyClaimLaneArgs) -> Result<String> {
    let guarded = load_run(&args.guarded_run)?;
    let claim = load_run(&args.claim_run)?;
    if claim.steps.is_empty() {
        bail!(
            "claim run has no stored step summaries: {}",
            args.claim_run.display()
        );
    }

    let audit = build_claim_lane_audit(&guarded, &claim);
    let accepted_hash_parity = accepted_hash_parity_check(&guarded, &claim);
    let early_breadth = breadth_check(&claim, EARLY_GENERATED_TARGETS, true);
    let late_generated_floors = breadth_check(&claim, LATE_GENERATED_FLOORS, false);
    let runtime_threshold = runtime_threshold_check(&claim, args.runtime_threshold_ms);
    let manifest_completeness = manifest_completeness_check(&claim.manifest);

    let checks = ClaimCertificateChecks {
        accepted_hash_parity: SimpleCheck {
            status: pass_fail(accepted_hash_parity.status == "ready"),
            detail: if accepted_hash_parity.status == "ready" {
                "accepted hashes match guarded through step 15".to_owned()
            } else {
                "claim lane diverges from the guarded accepted hashes".to_owned()
            },
        },
        search_policy: SimpleCheck {
            status: pass_fail(audit.search_policy.status == "honest"),
            detail: if audit.search_policy.status == "honest" {
                "claim search policy matches the expected claim-specific metadata".to_owned()
            } else {
                "claim search policy metadata does not match the expected claim lane".to_owned()
            },
        },
        fallback_honesty: SimpleCheck {
            status: pass_fail(audit.fallback_honesty.status == "clear"),
            detail: if audit.fallback_honesty.status == "clear" {
                "no replay or non-claim fallback evidence detected".to_owned()
            } else {
                "fallback or replay evidence was detected in the claim run".to_owned()
            },
        },
        narrative_artifacts: SimpleCheck {
            status: pass_fail(audit.narrative_artifacts.status == "complete"),
            detail: if audit.narrative_artifacts.status == "complete" {
                "claim narrative artifacts are complete".to_owned()
            } else {
                "claim narrative artifacts are incomplete".to_owned()
            },
        },
        early_breadth,
        late_generated_floors,
        runtime_threshold,
        exact_screen_reason_completeness: CoverageCheck {
            status: pass_fail(audit.exact_screen_reasons.status == "complete"),
            detail: if audit.exact_screen_reasons.status == "complete" {
                "exact-screen reasons are fully persisted in step artifacts".to_owned()
            } else {
                "exact-screen reasons are incomplete in the stored step artifacts".to_owned()
            },
        },
        prune_class_completeness: CoverageCheck {
            status: pass_fail(audit.prune_classes.status == "complete"),
            detail: if audit.prune_classes.status == "complete" {
                "prune class counts are fully persisted in step artifacts".to_owned()
            } else {
                "prune class counts are incomplete in the stored step artifacts".to_owned()
            },
        },
        manifest_completeness,
    };

    let failing_checks = collect_failing_checks(&checks);
    let certificate = ClaimCertificate {
        certificate_version: 1,
        generated_utc: current_utc_timestamp(),
        status: if failing_checks.is_empty() {
            "ready".to_owned()
        } else {
            "attention".to_owned()
        },
        claim_run: args.claim_run.display().to_string(),
        guarded_run: args.guarded_run.display().to_string(),
        runtime_threshold_ms: args.runtime_threshold_ms,
        failing_checks,
        checks,
    };
    let text = render_certificate_text(&certificate);

    if let Some(path) = args.json_out.as_deref() {
        write_text(path, &(serde_json::to_string_pretty(&certificate)? + "\n"))?;
    }
    if let Some(path) = args.text_out.as_deref() {
        write_text(path, &text)?;
    }

    Ok(text)
}

fn pass_fail(value: bool) -> String {
    if value {
        "pass".to_owned()
    } else {
        "fail".to_owned()
    }
}

fn collect_failing_checks(checks: &ClaimCertificateChecks) -> Vec<String> {
    let mut failing = Vec::new();
    if checks.accepted_hash_parity.status == "fail" {
        failing.push("accepted_hash_parity".to_owned());
    }
    if checks.search_policy.status == "fail" {
        failing.push("search_policy".to_owned());
    }
    if checks.fallback_honesty.status == "fail" {
        failing.push("fallback_honesty".to_owned());
    }
    if checks.narrative_artifacts.status == "fail" {
        failing.push("narrative_artifacts".to_owned());
    }
    if checks.early_breadth.status == "fail" {
        failing.push("early_breadth".to_owned());
    }
    if checks.late_generated_floors.status == "fail" {
        failing.push("late_generated_floors".to_owned());
    }
    if checks.runtime_threshold.status == "fail" {
        failing.push("runtime_threshold".to_owned());
    }
    if checks.exact_screen_reason_completeness.status == "fail" {
        failing.push("exact_screen_reason_completeness".to_owned());
    }
    if checks.prune_class_completeness.status == "fail" {
        failing.push("prune_class_completeness".to_owned());
    }
    if checks.manifest_completeness.status == "fail" {
        failing.push("manifest_completeness".to_owned());
    }
    failing
}

fn render_certificate_text(certificate: &ClaimCertificate) -> String {
    let mut lines = vec![
        format!("Claim Certification: {}", certificate.status),
        format!("claim run: {}", certificate.claim_run),
        format!("guarded run: {}", certificate.guarded_run),
        format!(
            "runtime threshold ms: {}",
            certificate
                .runtime_threshold_ms
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_owned())
        ),
        format!(
            "accepted_hash_parity: {} - {}",
            certificate.checks.accepted_hash_parity.status,
            certificate.checks.accepted_hash_parity.detail
        ),
        format!(
            "search_policy: {} - {}",
            certificate.checks.search_policy.status, certificate.checks.search_policy.detail
        ),
        format!(
            "fallback_honesty: {} - {}",
            certificate.checks.fallback_honesty.status, certificate.checks.fallback_honesty.detail
        ),
        format!(
            "narrative_artifacts: {} - {}",
            certificate.checks.narrative_artifacts.status,
            certificate.checks.narrative_artifacts.detail
        ),
        format!(
            "early_breadth: {} - {}",
            certificate.checks.early_breadth.status, certificate.checks.early_breadth.detail
        ),
    ];
    lines.extend(
        certificate
            .checks
            .early_breadth
            .steps
            .iter()
            .map(|step| format!("  step {}: {}", step.step_index, step.diagnosis.summary)),
    );
    lines.push(format!(
        "late_generated_floors: {} - {}",
        certificate.checks.late_generated_floors.status,
        certificate.checks.late_generated_floors.detail
    ));
    lines.extend(
        certificate
            .checks
            .late_generated_floors
            .steps
            .iter()
            .map(|step| format!("  step {}: {}", step.step_index, step.diagnosis.summary)),
    );
    lines.push(format!(
        "runtime_threshold: {} - {}",
        certificate.checks.runtime_threshold.status, certificate.checks.runtime_threshold.detail
    ));
    lines.push(format!(
        "exact_screen_reason_completeness: {} - {}",
        certificate.checks.exact_screen_reason_completeness.status,
        certificate.checks.exact_screen_reason_completeness.detail
    ));
    lines.push(format!(
        "prune_class_completeness: {} - {}",
        certificate.checks.prune_class_completeness.status,
        certificate.checks.prune_class_completeness.detail
    ));
    lines.push(format!(
        "manifest_completeness: {} - {}",
        certificate.checks.manifest_completeness.status,
        certificate.checks.manifest_completeness.detail
    ));
    if !certificate.failing_checks.is_empty() {
        lines.push(format!(
            "failing checks: {}",
            certificate.failing_checks.join(", ")
        ));
    }
    format!("{}\n", lines.join("\n"))
}
