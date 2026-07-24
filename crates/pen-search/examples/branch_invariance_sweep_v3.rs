use pen_search::branch_invariance_sweep_v3::{
    emit_bi1_option_a_sweep_v3_create_new, issue_bi1_option_a_sweep_v3,
    render_bi1_option_a_sweep_v3, replay_bi1_option_a_sweep_v3,
};
use std::path::Path;

fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let (bundle, emitter_already_replayed_bundle) = match arguments.as_slice() {
        [] => (
            issue_bi1_option_a_sweep_v3().map_err(|error| error.to_string())?,
            false,
        ),
        [flag, directory] if flag == "--emit" => (
            emit_bi1_option_a_sweep_v3_create_new(Path::new(directory))
                .map_err(|error| error.to_string())?,
            true,
        ),
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example branch_invariance_sweep_v3 [--emit <directory>]"
                    .to_owned(),
            );
        }
    };
    if !emitter_already_replayed_bundle {
        let replay = replay_bi1_option_a_sweep_v3(&bundle);
        if !replay.valid {
            return Err(format!(
                "BI-1 Option-A sweep replay failed: {}",
                replay.errors.join("; ")
            ));
        }
    }
    println!("{}", render_bi1_option_a_sweep_v3(&bundle.manifest));
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("bi1-option-a-sweep-v3".to_owned())
        .stack_size(256 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("BI-1 Option-A sweep worker panicked".into()),
    }
}
