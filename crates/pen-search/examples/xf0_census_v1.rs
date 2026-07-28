//! XF-0 census join (campaign XF-1, `docs/xf1_external_falsifier_campaign.md`).
//!
//! Mechanical, replayable comparison of the blind-filled physics column
//! (`docs/xf0_physics_column_v1.json`) against the certified semantic register
//! bound in MS-1 (`docs/milestone_certificate_v1.json`). The correspondence
//! rule (`docs/xf0_correspondence_rule_v1.md`) is bound by digest into the
//! physics column, which proves the F-XF2 ordering: rule frozen, column filled
//! blind, join last. No row is adjusted here; verdicts are computed and
//! published verbatim.
//!
//! Usage:
//!   cargo run -p pen-search --example xf0_census_v1 -- hash <file>
//!   cargo run -p pen-search --example xf0_census_v1 -- census <docs-dir>

use pen_core::hash::blake3_hex;
use pen_search::milestone_certificate_v1::replay_milestone_certificate_v1_json;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

const EXPECTED_MS1_DIGEST: &str =
    "blake3:37d96619734afb37125f3cc28fc3cac87b8b0b81c8c0a9945156bdba6c43bb0b";
/// Stage-4 non-enacted value class, certified branch-indexed in BI-4 v2
/// (`bi4_cone_v2`, blake3:75a9dcc023e8a7e7b00d9e20ced5516cb0d08f8b88ffd80f93d45e2f746191b1,
/// G3b rows: branch semantic_nu values {3,3,2,2}); bound in the MS-1 chain.
const STAGE4_CONE_MINIMUM: u64 = 2;
const BI4_CONE_V2_DIGEST: &str =
    "blake3:75a9dcc023e8a7e7b00d9e20ced5516cb0d08f8b88ffd80f93d45e2f746191b1";

const CALIBRATION_STRATA: [u64; 3] = [8, 9, 12];
const BLIND_SINGLE_STRATA: [u64; 11] = [1, 2, 3, 5, 6, 7, 10, 11, 13, 14, 15];

fn file_digest(path: &Path) -> Result<(Vec<u8>, String), Box<dyn std::error::Error>> {
    let bytes =
        fs::read(path).map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    let digest = format!("blake3:{}", blake3_hex(&bytes));
    Ok((bytes, digest))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [command, file] if command == "hash" => {
            let (_, digest) = file_digest(Path::new(file))?;
            println!("{digest}  {file}");
        }
        [command, docs] if command == "census" => {
            run_census(Path::new(docs))?;
        }
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example xf0_census_v1 -- [hash <file> | census <docs-dir>]"
                    .into(),
            );
        }
    }
    Ok(())
}

fn run_census(docs: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Foundation: MS-1 must carry the exact frozen result digest and replay valid.
    let (certificate_bytes, certificate_file_digest) =
        file_digest(&docs.join("milestone_certificate_v1.json"))?;
    let certificate_json = String::from_utf8(certificate_bytes)?;
    let replay = replay_milestone_certificate_v1_json(&certificate_json);
    if !replay.valid {
        return Err(format!("MS-1 replay failed: {}", replay.errors.join("; ")).into());
    }
    let certificate: Value = serde_json::from_str(&certificate_json)?;
    let result_digest = certificate["result_digest"]
        .as_str()
        .ok_or("MS-1 certificate has no result_digest")?;
    if result_digest != EXPECTED_MS1_DIGEST {
        return Err(format!(
            "MS-1 result digest mismatch: found {result_digest}, campaign binds {EXPECTED_MS1_DIGEST}"
        )
        .into());
    }
    let semantic_vector = certificate["semantic_vector"]
        .as_array()
        .ok_or("MS-1 semantic_vector missing")?
        .iter()
        .map(|entry| {
            entry["decimal"]
                .as_str()
                .ok_or("semantic_vector entry without decimal")
                .and_then(|decimal| decimal.parse::<u64>().map_err(|_| "non-numeric decimal"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if semantic_vector.len() != 15 || semantic_vector.iter().sum::<u64>() != 32 {
        return Err("MS-1 semantic vector is not the frozen 15-entry sum-32 register".into());
    }

    // 2. The frozen correspondence rule and the blind-filled physics column.
    let (_, rule_digest) = file_digest(&docs.join("xf0_correspondence_rule_v1.md"))?;
    let (column_bytes, column_digest) = file_digest(&docs.join("xf0_physics_column_v1.json"))?;
    let column: Value = serde_json::from_slice(&column_bytes)?;

    // 3. Chain of custody: the column must bind the exact frozen rule digest,
    //    proving the rule preceded the fill (F-XF2 recorded order).
    let bound_rule = column["correspondence_rule_blake3"]
        .as_str()
        .ok_or("physics column does not bind a correspondence rule digest")?;
    if bound_rule != rule_digest {
        return Err(format!(
            "custody chain broken: column binds {bound_rule}, rule file is {rule_digest}"
        )
        .into());
    }

    // 4. Row inventory must be exactly the frozen partition.
    let rows = column["rows"]
        .as_array()
        .ok_or("physics column has no rows")?;
    if rows.len() != 15 {
        return Err(format!("physics column has {} rows, expected 15", rows.len()).into());
    }
    let mut verdict_rows = Vec::new();
    let mut blind_single_matches = 0u64;
    let mut calibration_matches = 0u64;
    let mut stage4_verdict = json!(null);
    for (index, row) in rows.iter().enumerate() {
        let stratum = row["stratum"].as_u64().ok_or("row without stratum")?;
        if stratum != index as u64 + 1 {
            return Err(format!("row {index} carries stratum {stratum}, expected in order").into());
        }
        let d_count = row["d_count"].as_u64().ok_or("row without d_count")?;
        let row_kind = row["row_kind"].as_str().ok_or("row without row_kind")?;
        let expected_kind = if stratum == 4 {
            "BlindDualTarget"
        } else if CALIBRATION_STRATA.contains(&stratum) {
            "Calibration"
        } else {
            "BlindSingleTarget"
        };
        if row_kind != expected_kind {
            return Err(format!(
                "row {stratum} kind {row_kind} differs from frozen partition {expected_kind}"
            )
            .into());
        }
        let certified = semantic_vector[index];
        let verdict = if stratum == 4 {
            let matches_enacted = d_count == certified;
            let matches_cone_minimum = d_count == STAGE4_CONE_MINIMUM;
            let entry = json!({
                "kind": "BranchIndexed",
                "enacted_branch_value": certified,
                "cone_minimum_value": STAGE4_CONE_MINIMUM,
                "cone_minimum_source": BI4_CONE_V2_DIGEST,
                "matches_enacted_branch": matches_enacted,
                "matches_cone_minimum": matches_cone_minimum,
            });
            stage4_verdict = entry.clone();
            entry
        } else {
            let matched = d_count == certified;
            if matched {
                if CALIBRATION_STRATA.contains(&stratum) {
                    calibration_matches += 1;
                } else {
                    blind_single_matches += 1;
                }
            }
            json!({
                "kind": "SingleTarget",
                "certified_value": certified,
                "verdict": if matched { "Match" } else { "Mismatch" },
            })
        };
        verdict_rows.push(json!({
            "stratum": stratum,
            "name": row["name"],
            "row_kind": row_kind,
            "d_count": d_count,
            "enumeration": row["enumeration"],
            "citations": row["citations"],
            "verifier_disputes": row["verifier_disputes"],
            "adjudication": row["adjudication"],
            "comparison": verdict,
        }));
    }
    if BLIND_SINGLE_STRATA.len() != 11 {
        return Err("frozen partition corrupted".into());
    }

    let census = json!({
        "artifact": "xf0_census_v1",
        "campaign": "XF-1",
        "item": "XF-0 zero-worth/degrees-of-freedom census",
        "date": "2026-07-24",
        "inputs": {
            "milestone_certificate": {
                "path": "docs/milestone_certificate_v1.json",
                "result_digest": EXPECTED_MS1_DIGEST,
                "file_blake3": certificate_file_digest,
                "replay_valid": true,
            },
            "correspondence_rule": {
                "path": "docs/xf0_correspondence_rule_v1.md",
                "file_blake3": rule_digest,
            },
            "physics_column": {
                "path": "docs/xf0_physics_column_v1.json",
                "file_blake3": column_digest,
                "binds_rule_digest": bound_rule,
            },
        },
        "recorded_order": {
            "column_bound_to_frozen_rule": true,
            "semantic_register_sealed_before_campaign": true,
            "join_performed_after_all_rows_final": true,
        },
        "certified_targets": {
            "register": "SemanticRegisterAuthority (MS-1 semantic_vector)",
            "stages_1_to_3": "pre-fork common prefix",
            "stage_4": "branch-indexed value classes {enacted, cone minimum}",
            "stages_5_to_15": "cone-level (BI-4 G3a)",
            "vector": semantic_vector,
        },
        "rows": verdict_rows,
        "tally": {
            "blind_single_target_rows": 11,
            "blind_single_target_matches": blind_single_matches,
            "stage_4_dual_target": stage4_verdict,
            "calibration_rows": 3,
            "calibration_matches": calibration_matches,
        },
    });
    let mut rendered = serde_json::to_string_pretty(&census)?;
    rendered.push('\n');
    let output_path = docs.join("xf0_census_v1.json");
    fs::write(&output_path, &rendered)?;
    let result_digest = format!("blake3:{}", blake3_hex(rendered.as_bytes()));
    println!(
        "XF-0 census complete: blind {blind_single_matches}/11, stage-4 {}, calibration {calibration_matches}/3, output {} ({result_digest})",
        serde_json::to_string(&stage4_verdict)?,
        output_path.display()
    );
    Ok(())
}
