//! Read-only T-SM1a development and replay harness.
//!
//! It emits no certificate and opens no downstream gate.

// The theorem module is also compiled directly by this example. Re-export
// its two already registered dependencies without recursively path-including
// their private dependency graphs.
mod chronological_slot_map_v3 {
    pub use pen_search::chronological_slot_map_v3::*;
}
mod contextual_formation_coherence_v3 {
    pub use pen_search::contextual_formation_coherence_v3::*;
}
#[path = "../src/t_sm1a_contextual_formation_v4.rs"]
mod t_sm1a_contextual_formation_v4;

use t_sm1a_contextual_formation_v4::{
    issue_t_sm1a_audit_v4, render_t_sm1a_audit_v4, replay_t_sm1a_audit_v4,
};

fn run() -> Result<(), String> {
    let audit = issue_t_sm1a_audit_v4().map_err(|error| error.to_string())?;
    let replay = replay_t_sm1a_audit_v4(&audit);
    if !replay.valid {
        return Err(format!("T-SM1a audit replay failed: {:?}", replay.errors));
    }
    println!("{}", render_t_sm1a_audit_v4(&audit));
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("t-sm1a-contextual-formation-v4".to_owned())
        .stack_size(256 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("T-SM1a worker panicked".into()),
    }
}
