//! XF-0b2 targeted re-examination of family `df4feb52882e`: sealing and
//! replay (campaign XF-1, `docs/xf0b2_df4feb52_reexamination_plan.md`).
//!
//! `seal` canonicalizes the draft (`xf0b2_df4feb52_v1.draft.json`), computes
//! the result digest over the sealed content, and writes the final artifact.
//!
//! `replay` is the plan's mutation falsifier. It
//!   1. recomputes the result digest (any edited verdict, attempt, citation
//!      or quorum row changes the canonical serialization and fails here);
//!   2. replays the MS-1 foundation and checks the campaign binding;
//!   3. **re-derives every Phase R-0 claim from the frozen kernel** and
//!      compares it against the sealed `r0` block — so the artifact cannot
//!      drift from the elaborator, and a kernel change invalidates the
//!      result rather than silently reinterpreting it (falsifier F-R1);
//!   4. enforces the R-1 recording discipline: all five registered leads
//!      disposed, every attempt carrying citations and either a translation
//!      or a named obstruction (F-R2);
//!   5. enforces the R-2 stability quorum: a `no_name_confirmed` disposition
//!      requires two independent armed analysts and an adversarial verifier
//!      concurring (F-R3);
//!   6. binds the parent XF-0b concordance by digest and checks that the
//!      subject row is still the one this study re-examined.
//!
//! Usage:
//!   cargo run -p pen-search --example xf0b2_df4feb52_v1 -- seal <docs-dir>
//!   cargo run -p pen-search --example xf0b2_df4feb52_v1 -- replay <docs-dir>

use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::typed_families::{
    CandidateExtractionOutcome, extract_candidate_families, predecessor_closure,
};
use pen_search::milestone_certificate_v1::replay_milestone_certificate_v1_json;
use pen_type::elaborate::{SealedSignature, elaborate_telescope, minimal_ambient_parameters};
use pen_type::normalize::normalize;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

const EXPECTED_MS1_DIGEST: &str =
    "blake3:37d96619734afb37125f3cc28fc3cac87b8b0b81c8c0a9945156bdba6c43bb0b";
const PARENT_CONCORDANCE_DIGEST: &str =
    "blake3:907b7a474dcc530fe5b332d3caec51758dc104ab3226e915a5c357f7388785d2";
const SUBJECT: &str = "df4feb52882e";
const SUBJECT_FAMILY_ID: &str =
    "blake3:df4feb52882ec5b6a56472960c340c6a16be3b8c48dd25d389f9b5dca8e9e62b";
const REGISTERED_LEADS: [&str; 5] = ["L1", "L2", "L3", "L4", "L5"];

type Fallible<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> Fallible<()> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [command, docs] if command == "seal" => seal(Path::new(docs)),
        [command, docs] if command == "replay" => replay(Path::new(docs)),
        [command] if command == "r0" => emit_r0(),
        _ => Err(
            "usage: cargo run -p pen-search --example xf0b2_df4feb52_v1 -- [seal <docs-dir> | replay <docs-dir> | r0]"
                .into(),
        ),
    }
}

fn sealed_digest(sealed: &Value) -> Fallible<String> {
    let canonical = serde_json::to_string(sealed)?;
    Ok(format!("blake3:{}", blake3_hex(canonical.as_bytes())))
}

fn seal(docs: &Path) -> Fallible<()> {
    let draft: Value =
        serde_json::from_slice(&fs::read(docs.join("xf0b2_df4feb52_v1.draft.json"))?)?;
    let sealed = draft
        .get("sealed")
        .ok_or("draft has no `sealed` value")?
        .clone();
    let digest = sealed_digest(&sealed)?;
    let artifact = json!({
        "artifact": "xf0b2_df4feb52_v1",
        "campaign": "XF-1",
        "result_digest": digest,
        "sealed": sealed,
    });
    let mut rendered = serde_json::to_string_pretty(&artifact)?;
    rendered.push('\n');
    fs::write(docs.join("xf0b2_df4feb52_v1.json"), &rendered)?;
    println!("XF-0b2 sealed: {digest}");
    Ok(())
}

/// Everything Phase R-0 asserts, recomputed from the frozen kernel.
struct DerivedR0 {
    signature_digest: String,
    predecessor_signature_digest: String,
    predecessor_closure_digest: String,
    stage14_candidate_hash: String,
    ambient_parameters: u32,
    clause_expr: String,
    clause_normal_form: String,
    beta_steps_to_normal_form: u32,
    coarse_assumptions: u32,
    app_nodes: u32,
    var_resolutions: Vec<String>,
    family_id: String,
    canonical_normal_form: String,
    parameters: Vec<String>,
    marginal_no_closure_preimage: bool,
    instance_count: usize,
    lam_in_pi_domain_occurrences: Vec<String>,
    sealed_computation_role_clauses: u32,
}

fn derive_r0() -> Fallible<DerivedR0> {
    let signature = SealedSignature::genesis_del_h15();
    let t14 = Telescope::reference(14);
    let elaboration = elaborate_telescope(&signature, &t14, 13)
        .map_err(|failure| format!("stage-14 elaboration failed: {failure}"))?;
    let ambient = minimal_ambient_parameters(&t14);
    let clause = &elaboration.clauses[3];
    let expr = &t14.clauses[3].expr;
    let base = ambient + 3;
    let normalized = normalize(expr, base, 256)?;

    // Var resolution under the frozen scope `[ambient, prior fields, binders]`.
    let var_resolutions = vec![
        format!("Lam body Var 1 -> {}", resolve(1, ambient, 3)),
        format!("Sigma domain Var 1 -> {}", resolve(1, ambient, 3)),
        format!("Sigma codomain Var 2 -> {}", resolve(2, ambient, 3)),
    ];

    // The certified family ids are minted against the PREDECESSOR signature.
    let prefix: Vec<(u32, Telescope)> = (1..=13).map(|s| (s, Telescope::reference(s))).collect();
    let prefix_signature = SealedSignature::from_telescopes(prefix);
    let closure = predecessor_closure(&prefix_signature)?;
    let extraction = match extract_candidate_families(&prefix_signature, &closure, &t14, 13) {
        CandidateExtractionOutcome::Extracted(extraction) => extraction,
        CandidateExtractionOutcome::KernelInvalid { failure } => {
            return Err(format!("stage 14 is kernel-invalid: {failure}").into());
        }
    };
    let family = extraction
        .families
        .iter()
        .find(|family| family.id.as_str() == SUBJECT_FAMILY_ID)
        .ok_or("the subject family is not among the extracted stage-14 families")?;

    let mut lam_in_pi_domain_occurrences = Vec::new();
    for step in 1..=15u32 {
        let telescope = Telescope::reference(step);
        for (index, clause) in telescope.clauses.iter().enumerate() {
            if has_lam_in_pi_domain(&clause.expr) {
                lam_in_pi_domain_occurrences.push(format!("step {step} clause {index}"));
            }
        }
    }

    let sealed_computation_role_clauses = signature
        .entries()
        .iter()
        .map(|entry| u32::from(entry.exported_computation_clauses))
        .sum();

    Ok(DerivedR0 {
        signature_digest: signature.digest().to_string(),
        predecessor_signature_digest: prefix_signature.digest().to_string(),
        predecessor_closure_digest: closure.digest.clone(),
        stage14_candidate_hash: signature.entry(14).unwrap().candidate_hash.clone(),
        ambient_parameters: elaboration.ambient_parameters,
        clause_expr: format!("{expr:?}"),
        clause_normal_form: format!("{:?}", clause.normal_form),
        beta_steps_to_normal_form: normalized.steps,
        coarse_assumptions: clause.coarse_assumptions,
        app_nodes: count_apps(expr),
        var_resolutions,
        family_id: family.id.as_str().to_string(),
        canonical_normal_form: format!("{:?}", family.presentation.canonical_normal_form),
        parameters: family
            .presentation
            .parameters
            .iter()
            .map(|sort| format!("{sort:?}"))
            .collect(),
        marginal_no_closure_preimage: family.marginality.is_marginal(),
        instance_count: family.instances.len(),
        lam_in_pi_domain_occurrences,
        sealed_computation_role_clauses,
    })
}

/// Emit the `r0` block exactly as `replay` will recompute it, so the draft
/// carries kernel-derived values rather than transcribed ones.
fn emit_r0() -> Fallible<()> {
    let d = derive_r0()?;
    let block = json!({
        "signature_digest": d.signature_digest,
        "predecessor_signature_digest": d.predecessor_signature_digest,
        "predecessor_closure_digest": d.predecessor_closure_digest,
        "stage14_candidate_hash": d.stage14_candidate_hash,
        "ambient_parameters": d.ambient_parameters,
        "clause_expr": d.clause_expr,
        "clause_normal_form": d.clause_normal_form,
        "beta_steps_to_normal_form": d.beta_steps_to_normal_form,
        "coarse_assumptions": d.coarse_assumptions,
        "app_nodes": d.app_nodes,
        "var_resolutions": d.var_resolutions,
        "family_id": d.family_id,
        "canonical_normal_form": d.canonical_normal_form,
        "parameters": d.parameters,
        "marginal_no_closure_preimage": d.marginal_no_closure_preimage,
        "instance_count": d.instance_count,
        "lam_in_pi_domain_occurrences": d.lam_in_pi_domain_occurrences,
        "sealed_computation_role_clauses": d.sealed_computation_role_clauses,
    });
    println!("{}", serde_json::to_string_pretty(&block)?);
    Ok(())
}

fn replay(docs: &Path) -> Fallible<()> {
    // 1. Digest integrity.
    let artifact: Value = serde_json::from_slice(&fs::read(docs.join("xf0b2_df4feb52_v1.json"))?)?;
    let sealed = artifact
        .get("sealed")
        .ok_or("artifact has no `sealed` value")?;
    let recorded = artifact["result_digest"]
        .as_str()
        .ok_or("artifact has no result_digest")?;
    let recomputed = sealed_digest(sealed)?;
    if recorded != recomputed {
        return Err(
            format!("mutation detected: recorded {recorded}, recomputed {recomputed}").into(),
        );
    }

    // 2. Foundation.
    let certificate_json = fs::read_to_string(docs.join("milestone_certificate_v1.json"))?;
    let ms1 = replay_milestone_certificate_v1_json(&certificate_json);
    if !ms1.valid {
        return Err(format!("MS-1 replay failed: {}", ms1.errors.join("; ")).into());
    }
    let certificate: Value = serde_json::from_str(&certificate_json)?;
    if certificate["result_digest"].as_str() != Some(EXPECTED_MS1_DIGEST) {
        return Err("MS-1 result digest differs from the campaign binding".into());
    }

    // 3. Phase R-0 re-derived from the kernel (F-R1: the study proceeds on
    //    the elaboration, so the elaboration must still be what it says).
    let derived = derive_r0()?;
    let r0 = sealed
        .get("r0")
        .ok_or("sealed artifact has no `r0` block")?;
    check_str(r0, "signature_digest", &derived.signature_digest)?;
    check_str(
        r0,
        "predecessor_signature_digest",
        &derived.predecessor_signature_digest,
    )?;
    check_str(
        r0,
        "predecessor_closure_digest",
        &derived.predecessor_closure_digest,
    )?;
    check_str(
        r0,
        "stage14_candidate_hash",
        &derived.stage14_candidate_hash,
    )?;
    check_str(r0, "clause_expr", &derived.clause_expr)?;
    check_str(r0, "clause_normal_form", &derived.clause_normal_form)?;
    check_str(r0, "family_id", &derived.family_id)?;
    check_str(r0, "canonical_normal_form", &derived.canonical_normal_form)?;
    check_u64(
        r0,
        "ambient_parameters",
        u64::from(derived.ambient_parameters),
    )?;
    check_u64(
        r0,
        "beta_steps_to_normal_form",
        u64::from(derived.beta_steps_to_normal_form),
    )?;
    check_u64(
        r0,
        "coarse_assumptions",
        u64::from(derived.coarse_assumptions),
    )?;
    check_u64(r0, "app_nodes", u64::from(derived.app_nodes))?;
    check_u64(r0, "instance_count", derived.instance_count as u64)?;
    check_u64(
        r0,
        "sealed_computation_role_clauses",
        u64::from(derived.sealed_computation_role_clauses),
    )?;
    check_bool(
        r0,
        "marginal_no_closure_preimage",
        derived.marginal_no_closure_preimage,
    )?;
    check_strings(r0, "var_resolutions", &derived.var_resolutions)?;
    check_strings(r0, "parameters", &derived.parameters)?;
    check_strings(
        r0,
        "lam_in_pi_domain_occurrences",
        &derived.lam_in_pi_domain_occurrences,
    )?;
    // The irreducibility certificate is not decoration: it must still hold.
    if derived.app_nodes != 0 || derived.beta_steps_to_normal_form != 0 {
        return Err(
            "R-0.2 irreducibility certificate no longer holds: the clause has a redex".into(),
        );
    }
    if derived.sealed_computation_role_clauses != 0 {
        return Err(
            "R-0.2 certificate broken: a sealed entry now exports a computation-role clause".into(),
        );
    }

    // 4. R-1 recording discipline (F-R2).
    let attempts = sealed["r1"]["lead_attempts"]
        .as_array()
        .ok_or("sealed artifact has no `r1.lead_attempts` array")?;
    for lead in REGISTERED_LEADS {
        let row = attempts
            .iter()
            .find(|row| row["lead"].as_str() == Some(lead))
            .ok_or_else(|| format!("registered lead {lead} is not disposed (F-R3)"))?;
        let disposition = row["lead_verdict"].as_str().unwrap_or("");
        if !matches!(disposition, "named" | "no_name") {
            return Err(format!("lead {lead} carries no admissible verdict").into());
        }
        let targets = row["targets_tried"]
            .as_array()
            .ok_or_else(|| format!("lead {lead} records no targets"))?;
        if targets.is_empty() {
            return Err(format!("lead {lead} records an empty target list").into());
        }
        for target in targets {
            let citations = target["citations"].as_array();
            if citations.map_or(true, Vec::is_empty) {
                return Err(format!(
                    "F-R2 violation: a target under lead {lead} carries no citations"
                )
                .into());
            }
            match target["outcome"].as_str() {
                Some("named") => {
                    if target["translation_attempt"]
                        .as_str()
                        .map_or(true, str::is_empty)
                    {
                        return Err(format!(
                            "F-R2 violation: a `named` target under lead {lead} carries no translation"
                        )
                        .into());
                    }
                }
                Some("obstruction") => {
                    if target["obstruction"].as_str().map_or(true, str::is_empty) {
                        return Err(format!(
                            "F-R2 violation: an `obstruction` target under lead {lead} names no obstruction"
                        )
                        .into());
                    }
                }
                _ => {
                    return Err(format!(
                        "a target under lead {lead} has an outcome outside {{named, obstruction}}"
                    )
                    .into());
                }
            }
        }
    }

    // 5. R-2 stability quorum (F-R3).
    let disposition = sealed["r2"]["disposition"]
        .as_str()
        .ok_or("sealed artifact has no `r2.disposition`")?;
    if !matches!(disposition, "named" | "no_name_confirmed" | "unstable") {
        return Err(format!("disposition `{disposition}` is outside the registered three").into());
    }
    let quorum = sealed["r2"]["quorum"]
        .as_array()
        .ok_or("sealed artifact has no `r2.quorum` array")?;
    if disposition == "no_name_confirmed" {
        let analysts: Vec<&Value> = quorum
            .iter()
            .filter(|row| row["role"].as_str() == Some("independent_armed_analyst"))
            .collect();
        let verifiers: Vec<&Value> = quorum
            .iter()
            .filter(|row| row["role"].as_str() == Some("adversarial_verifier"))
            .collect();
        if analysts.len() < 2 {
            return Err(
                "F-R3 violation: `no_name_confirmed` without two independent armed analysts".into(),
            );
        }
        if verifiers.is_empty() {
            return Err(
                "F-R3 violation: `no_name_confirmed` without an adversarial verifier".into(),
            );
        }
        for row in analysts.iter().chain(verifiers.iter()) {
            if row["verdict"].as_str() != Some("no_name") {
                return Err(format!(
                    "F-R3 violation: quorum member `{}` does not concur",
                    row["id"].as_str().unwrap_or("?")
                )
                .into());
            }
        }
    }

    // 6. Parent binding: the study this one re-examines, and its subject row.
    let parent: Value = serde_json::from_slice(&fs::read(docs.join("xf0b_concordance_v1.json"))?)?;
    if parent["result_digest"].as_str() != Some(PARENT_CONCORDANCE_DIGEST) {
        return Err(
            "the XF-0b concordance no longer carries the digest this study re-examines".into(),
        );
    }
    let parent_row = parent["sealed"]["rows"]
        .as_array()
        .ok_or("parent concordance has no rows")?
        .iter()
        .find(|row| row["family_short_id"].as_str() == Some(SUBJECT))
        .ok_or("the subject family is absent from the parent concordance")?;
    if parent_row["verdict"].as_str() != Some("no_name") {
        return Err(
            "the parent concordance's subject row is no longer the NO-NAME survivor".into(),
        );
    }

    println!(
        "XF-0b2 replay valid: digest {recorded}; R-0 re-derived from the kernel (ambient {}, coarse {}, {} beta steps, family {}); {} leads disposed; disposition `{disposition}` with a {}-member quorum; parent concordance bound.",
        derived.ambient_parameters,
        derived.coarse_assumptions,
        derived.beta_steps_to_normal_form,
        &derived.family_id[..20],
        attempts.len(),
        quorum.len(),
    );
    Ok(())
}

fn check_str(block: &Value, key: &str, expected: &str) -> Fallible<()> {
    let found = block[key]
        .as_str()
        .ok_or_else(|| format!("r0 block has no string `{key}`"))?;
    if found != expected {
        return Err(format!(
            "R-0 drift at `{key}`: artifact records `{found}`, the kernel derives `{expected}`"
        )
        .into());
    }
    Ok(())
}

fn check_u64(block: &Value, key: &str, expected: u64) -> Fallible<()> {
    let found = block[key]
        .as_u64()
        .ok_or_else(|| format!("r0 block has no number `{key}`"))?;
    if found != expected {
        return Err(format!(
            "R-0 drift at `{key}`: artifact records {found}, the kernel derives {expected}"
        )
        .into());
    }
    Ok(())
}

fn check_bool(block: &Value, key: &str, expected: bool) -> Fallible<()> {
    let found = block[key]
        .as_bool()
        .ok_or_else(|| format!("r0 block has no boolean `{key}`"))?;
    if found != expected {
        return Err(format!(
            "R-0 drift at `{key}`: artifact records {found}, the kernel derives {expected}"
        )
        .into());
    }
    Ok(())
}

fn check_strings(block: &Value, key: &str, expected: &[String]) -> Fallible<()> {
    let found: Vec<&str> = block[key]
        .as_array()
        .ok_or_else(|| format!("r0 block has no array `{key}`"))?
        .iter()
        .map(|entry| entry.as_str().unwrap_or(""))
        .collect();
    let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
    if found != expected {
        return Err(format!(
            "R-0 drift at `{key}`: artifact records {found:?}, the kernel derives {expected:?}"
        )
        .into());
    }
    Ok(())
}

fn resolve(level: u32, ambient: u32, priors: u32) -> String {
    if level == 0 {
        "invalid".to_string()
    } else if level <= ambient {
        format!("ambient parameter #{level}")
    } else if level <= ambient + priors {
        format!("field of clause {}", level - ambient - 1)
    } else {
        format!("local binder #{}", level - ambient - priors)
    }
}

fn count_apps(expr: &Expr) -> u32 {
    match expr {
        Expr::App(a, b) => 1 + count_apps(a) + count_apps(b),
        Expr::Pi(a, b) | Expr::Sigma(a, b) => count_apps(a) + count_apps(b),
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => count_apps(inner),
        Expr::Id(a, b, c) => count_apps(a) + count_apps(b) + count_apps(c),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => 0,
    }
}

fn has_lam_in_pi_domain(expr: &Expr) -> bool {
    match expr {
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            matches!(domain.as_ref(), Expr::Lam(_))
                || has_lam_in_pi_domain(domain)
                || has_lam_in_pi_domain(codomain)
        }
        Expr::App(a, b) => has_lam_in_pi_domain(a) || has_lam_in_pi_domain(b),
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => has_lam_in_pi_domain(inner),
        Expr::Id(a, b, c) => {
            has_lam_in_pi_domain(a) || has_lam_in_pi_domain(b) || has_lam_in_pi_domain(c)
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

// `ClauseRole` participates in the kernel-derived prior-role vector used by
// the extractor; the import keeps that dependency explicit at the call site.
const _: Option<ClauseRole> = None;
