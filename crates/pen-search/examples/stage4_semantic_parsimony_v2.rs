use pen_search::bi0_semantic_register_v6::{
    BI0_SEMANTIC_REGISTER_V6_CERTIFICATE_NAME, Bi0SemanticRegisterV6Artifact,
    issue_bi0_stage4_opening_capability_v6,
};
use pen_search::stage4_semantic_parsimony_v2::{
    emit_stage4_semantic_parsimony_v2_create_new, issue_stage4_semantic_parsimony_v2,
    replay_stage4_semantic_parsimony_v2_json,
};
use std::path::Path;

fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments
        .first()
        .is_some_and(|argument| argument == "create-new")
    {
        let Some(directory) = arguments.get(1) else {
            eprintln!("usage: stage4_semantic_parsimony_v2 create-new <directory>");
            std::process::exit(2);
        };
        // The previously emitted BI-0 artifact supplies only the claimed
        // capability.  The Stage-4 theorem replays that capability against
        // the current source-first issuer before doing any cone work.
        let bi0_path = Path::new(directory).join(BI0_SEMANTIC_REGISTER_V6_CERTIFICATE_NAME);
        let bi0_json = std::fs::read_to_string(&bi0_path).unwrap_or_else(|error| {
            eprintln!("could not read {}: {error}", bi0_path.display());
            std::process::exit(1);
        });
        let bi0_artifact: Bi0SemanticRegisterV6Artifact = serde_json::from_str(&bi0_json)
            .unwrap_or_else(|error| {
                eprintln!("could not parse {}: {error}", bi0_path.display());
                std::process::exit(1);
            });
        let opening = &bi0_artifact.stage4_opening_capability;
        match emit_stage4_semantic_parsimony_v2_create_new(opening, Path::new(directory)) {
            Ok(certificate) => {
                println!(
                    "outcome={} gate={} audit={} branch={} divergence={} digest={}",
                    certificate.outcome,
                    certificate.gate_sealed_before_blind_audit,
                    certificate.authorized_stage4_cone_audit,
                    certificate.authorized_bi1_branch_execution,
                    certificate.divergence_requires_versioned_adjudication,
                    certificate.result_digest,
                );
            }
            Err(error) => {
                eprintln!("Stage-4 v2 create-new failed: {error}");
                std::process::exit(1);
            }
        }
        return;
    }
    if arguments
        .first()
        .is_some_and(|argument| argument == "replay")
    {
        let Some(path) = arguments.get(1) else {
            eprintln!("usage: stage4_semantic_parsimony_v2 replay <json-path>");
            std::process::exit(2);
        };
        let json = std::fs::read_to_string(path).unwrap_or_else(|error| {
            eprintln!("could not read {path}: {error}");
            std::process::exit(1);
        });
        let replay = replay_stage4_semantic_parsimony_v2_json(&json);
        println!("{}", serde_json::to_string_pretty(&replay).unwrap());
        if !replay.valid {
            std::process::exit(1);
        }
        return;
    }

    let opening = issue_bi0_stage4_opening_capability_v6().unwrap_or_else(|error| {
        eprintln!("BI-0 v6 did not issue the Stage-4 opening: {error}");
        std::process::exit(1);
    });
    match issue_stage4_semantic_parsimony_v2(&opening) {
        Ok(certificate) => println!(
            "{}",
            serde_json::to_string_pretty(&certificate).expect("Stage-4 v2 certificate serializes")
        ),
        Err(error) => {
            eprintln!("Stage-4 v2 audit failed: {error}");
            std::process::exit(1);
        }
    }
}
