use pen_search::chronological_slot_map_v3::{
    emit_chronological_slot_map_v3_create_new, issue_chronological_slot_map_audit_v3,
    render_chronological_slot_map_audit_v3, replay_chronological_slot_map_audit_v3,
    replay_chronological_slot_map_v3_directory,
};
use std::path::Path;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if let [command, directory] = args.as_slice() {
        let replay = match command.as_str() {
            "create-new" => emit_chronological_slot_map_v3_create_new(Path::new(directory))
                .expect("v3 create-new emission"),
            "replay" => replay_chronological_slot_map_v3_directory(Path::new(directory)),
            other => panic!("unknown command {other}; use create-new or replay"),
        };
        if !replay.valid {
            panic!("v3 artifact failed replay: {}", replay.errors.join("; "));
        }
        println!(
            "valid={} f_sm1={} sealed_derived={} sealed_gaps={}",
            replay.valid,
            replay.f_sm1_passed,
            replay.sealed_discharge_derived_count,
            replay.sealed_discharge_named_gap_count
        );
        return;
    }
    if !args.is_empty() {
        panic!("usage: chronological_slot_map_v3 [create-new|replay DIRECTORY]");
    }
    let audit = issue_chronological_slot_map_audit_v3().expect("v3 in-memory audit issues");
    let replay = replay_chronological_slot_map_audit_v3(&audit);
    if !replay.valid {
        panic!(
            "v3 in-memory audit failed replay: {}",
            replay.errors.join("; ")
        );
    }
    print!("{}", render_chronological_slot_map_audit_v3(&audit));
}
