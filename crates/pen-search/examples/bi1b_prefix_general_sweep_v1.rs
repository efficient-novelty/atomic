use pen_search::bi1b_prefix_general_sweep_v1::{
    emit_bi1b_prefix_general_sweep_v1_create_new, issue_bi1b_prefix_general_sweep_v1,
    render_bi1b_sweep_v1, replay_bi1b_prefix_general_sweep_v1,
};
use std::path::Path;

fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let (bundle, emitter_already_replayed) = match arguments.as_slice() {
        [] => (
            issue_bi1b_prefix_general_sweep_v1().map_err(|error| error.to_string())?,
            false,
        ),
        [flag, directory] if flag == "--emit" => (
            emit_bi1b_prefix_general_sweep_v1_create_new(Path::new(directory))
                .map_err(|error| error.to_string())?,
            true,
        ),
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example bi1b_prefix_general_sweep_v1 [--emit <directory>]"
                    .to_owned(),
            );
        }
    };
    if !emitter_already_replayed {
        let replay = replay_bi1b_prefix_general_sweep_v1(&bundle);
        if !replay.valid {
            return Err(format!(
                "BI-1b sweep replay failed: {}",
                replay.errors.join("; ")
            ));
        }
    }
    println!("{}", render_bi1b_sweep_v1(&bundle.sweep));
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("bi1b-prefix-general-sweep-v1".to_owned())
        .stack_size(512 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("BI-1b prefix-general sweep worker panicked".into()),
    }
}
