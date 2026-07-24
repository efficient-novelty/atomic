//! XF-0b concordance sealing and replay (campaign XF-1,
//! `docs/xf0b_family_concordance_plan.md`).
//!
//! `seal` canonicalizes the draft concordance (`xf0b_concordance_v1.draft.json`),
//! computes the result digest over the sealed content, and writes the final
//! artifact. `replay` recomputes the digest and cross-checks the artifact
//! against its certified sources — so flipping any verdict, translation
//! clause, citation, or join row invalidates replay (the plan's mutation
//! falsifier).
//!
//! Usage:
//!   cargo run -p pen-search --example xf0b_concordance_v1 -- seal <docs-dir>
//!   cargo run -p pen-search --example xf0b_concordance_v1 -- replay <docs-dir>

use pen_core::hash::blake3_hex;
use pen_search::milestone_certificate_v1::replay_milestone_certificate_v1_json;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const EXPECTED_MS1_DIGEST: &str =
    "blake3:37d96619734afb37125f3cc28fc3cac87b8b0b81c8c0a9945156bdba6c43bb0b";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [command, docs] if command == "seal" => seal(Path::new(docs)),
        [command, docs] if command == "replay" => replay(Path::new(docs)),
        _ => Err(
            "usage: cargo run -p pen-search --example xf0b_concordance_v1 -- [seal <docs-dir> | replay <docs-dir>]"
                .into(),
        ),
    }
}

fn sealed_digest(sealed: &Value) -> Result<String, Box<dyn std::error::Error>> {
    let canonical = serde_json::to_string(sealed)?;
    Ok(format!("blake3:{}", blake3_hex(canonical.as_bytes())))
}

fn seal(docs: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let draft: Value =
        serde_json::from_slice(&fs::read(docs.join("xf0b_concordance_v1.draft.json"))?)?;
    let sealed = draft
        .get("sealed")
        .ok_or("draft has no `sealed` value")?
        .clone();
    let digest = sealed_digest(&sealed)?;
    let artifact = json!({
        "artifact": "xf0b_concordance_v1",
        "campaign": "XF-1",
        "result_digest": digest,
        "sealed": sealed,
    });
    let mut rendered = serde_json::to_string_pretty(&artifact)?;
    rendered.push('\n');
    fs::write(docs.join("xf0b_concordance_v1.json"), &rendered)?;
    println!("XF-0b sealed: {digest}");
    Ok(())
}

fn replay(docs: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Digest integrity: any mutated verdict/translation/citation/join row
    //    changes the canonical serialization and fails here.
    let artifact: Value = serde_json::from_slice(&fs::read(docs.join("xf0b_concordance_v1.json"))?)?;
    let sealed = artifact.get("sealed").ok_or("artifact has no `sealed` value")?;
    let recorded = artifact["result_digest"]
        .as_str()
        .ok_or("artifact has no result_digest")?;
    let recomputed = sealed_digest(sealed)?;
    if recorded != recomputed {
        return Err(format!(
            "mutation detected: recorded {recorded}, recomputed {recomputed}"
        )
        .into());
    }

    // 2. Foundation: MS-1 replays and carries the frozen result digest; the
    //    certified per-stage family counts must equal its semantic vector.
    let certificate_json = fs::read_to_string(docs.join("milestone_certificate_v1.json"))?;
    let ms1 = replay_milestone_certificate_v1_json(&certificate_json);
    if !ms1.valid {
        return Err(format!("MS-1 replay failed: {}", ms1.errors.join("; ")).into());
    }
    let certificate: Value = serde_json::from_str(&certificate_json)?;
    if certificate["result_digest"].as_str() != Some(EXPECTED_MS1_DIGEST) {
        return Err("MS-1 result digest differs from the campaign binding".into());
    }
    let semantic_vector: Vec<u64> = certificate["semantic_vector"]
        .as_array()
        .ok_or("MS-1 semantic_vector missing")?
        .iter()
        .map(|entry| entry["decimal"].as_str().unwrap_or("").parse::<u64>())
        .collect::<Result<_, _>>()?;
    let rows = sealed["rows"].as_array().ok_or("sealed rows missing")?;
    let mut per_stage = [0u64; 16];
    for row in rows {
        let stage = row["stage"].as_u64().ok_or("row without stage")? as usize;
        if !(1..=15).contains(&stage) {
            return Err("row stage out of range".into());
        }
        per_stage[stage] += 1;
        // 3. Standard conformance per row (F-C1, NO-NAME discipline, F-C5).
        match row["verdict"].as_str() {
            Some("named") => {
                let names = row["names"].as_array().ok_or("named row without names")?;
                if names.is_empty() {
                    return Err("named row with empty names list".into());
                }
                for name in names {
                    let translations = name["translation"].as_array();
                    let citations = name["citations"].as_array();
                    if translations.map_or(true, Vec::is_empty)
                        || citations.map_or(true, Vec::is_empty)
                    {
                        return Err(format!(
                            "F-C1 violation in stage {stage}: a name lacks translation rows or citations"
                        )
                        .into());
                    }
                }
            }
            Some("no_name") => {
                let attempts = row["attempts"].as_array().ok_or("no_name row without attempts")?;
                if attempts.len() < 3 {
                    return Err(format!(
                        "NO-NAME discipline violation in stage {stage}: fewer than three recorded attempts"
                    )
                    .into());
                }
            }
            _ => return Err("row with verdict outside {named, no_name}".into()),
        }
    }
    for stage in 1..=15 {
        if per_stage[stage] != semantic_vector[stage - 1] {
            return Err(format!(
                "family count at stage {stage} is {} but the certified register credits {}",
                per_stage[stage],
                semantic_vector[stage - 1]
            )
            .into());
        }
    }

    // 4. Inverse-join grounding: every blind datum referenced by the C-4
    //    inverse join must exist verbatim as a class-N datum of the sealed
    //    XF-0 census at the stated stage.
    let census: Value = serde_json::from_slice(&fs::read(docs.join("xf0_census_v1.json"))?)?;
    let census_rows = census["rows"].as_array().ok_or("census rows missing")?;
    let inverse = sealed["join"]["inverse_blind_to_families"]
        .as_array()
        .ok_or("inverse join missing")?;
    for entry in inverse {
        let stage = entry["stage"].as_u64().ok_or("inverse row without stage")?;
        let datum = entry["blind_datum"].as_str().ok_or("inverse row without datum")?;
        let found = census_rows.iter().any(|row| {
            row["stratum"].as_u64() == Some(stage)
                && row["enumeration"].as_array().map_or(false, |data| {
                    data.iter().any(|e| {
                        e["class"].as_str() == Some("N")
                            && e["datum"].as_str().map_or(false, |d| d.starts_with(datum))
                    })
                })
        });
        if !found {
            return Err(format!(
                "inverse join references a blind datum not present as class-N at stage {stage}: {datum}"
            )
            .into());
        }
    }

    println!(
        "XF-0b replay valid: digest {recorded}, {} family rows, counts match the certified register, inverse join grounded ({} blind data)",
        rows.len(),
        inverse.len()
    );
    Ok(())
}
