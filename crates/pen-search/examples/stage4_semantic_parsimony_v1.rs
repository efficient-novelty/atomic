use pen_search::stage4_semantic_parsimony_v1::{
    emit_stage4_semantic_parsimony_v1_create_new, issue_stage4_semantic_parsimony_v1,
    replay_stage4_semantic_parsimony_v1_json,
};
use std::path::Path;

fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.first().is_some_and(|argument| argument == "emit") {
        let Some(path) = arguments.get(1) else {
            eprintln!("usage: stage4_semantic_parsimony_v1 emit <create-new-json-path>");
            std::process::exit(2);
        };
        match emit_stage4_semantic_parsimony_v1_create_new(Path::new(path)) {
            Ok(certificate) => {
                eprintln!("created {path}: {}", certificate.result_digest);
                return;
            }
            Err(error) => {
                eprintln!("Stage-4 semantic parsimony create-new failed: {error}");
                std::process::exit(1);
            }
        }
    }
    if arguments
        .first()
        .is_some_and(|argument| argument == "replay")
    {
        let Some(path) = arguments.get(1) else {
            eprintln!("usage: stage4_semantic_parsimony_v1 replay <json-path>");
            std::process::exit(2);
        };
        let json = std::fs::read_to_string(path).unwrap_or_else(|error| {
            eprintln!("could not read {path}: {error}");
            std::process::exit(1);
        });
        let errors = replay_stage4_semantic_parsimony_v1_json(&json);
        if errors.is_empty() {
            println!("replay valid");
            return;
        }
        for error in errors {
            eprintln!("{error}");
        }
        std::process::exit(1);
    }

    match issue_stage4_semantic_parsimony_v1() {
        Ok(certificate) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&certificate)
                    .expect("Stage-4 semantic parsimony certificate serializes")
            );
        }
        Err(error) => {
            eprintln!("Stage-4 semantic parsimony audit failed: {error}");
            std::process::exit(1);
        }
    }
}
