use pen_search::bi2_branch_finales_v1::{
    emit_bi2_four_branch_finales_v1_create_new, issue_bi2_four_branch_finales_v1,
    render_bi2_four_branch_finales_v1, replay_bi2_four_branch_finales_v1,
};
use std::path::Path;

fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let (bundle, emitter_already_replayed) = match arguments.as_slice() {
        [] => (
            issue_bi2_four_branch_finales_v1().map_err(|error| error.to_string())?,
            false,
        ),
        [flag, directory] if flag == "--emit" => (
            emit_bi2_four_branch_finales_v1_create_new(Path::new(directory))
                .map_err(|error| error.to_string())?,
            true,
        ),
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example bi2_branch_finales_v1 [--emit <directory>]"
                    .to_owned(),
            );
        }
    };
    if !emitter_already_replayed {
        let replay = replay_bi2_four_branch_finales_v1(&bundle);
        if !replay.valid {
            return Err(format!(
                "BI-2 four-branch finale replay failed: {}",
                replay.errors.join("; ")
            ));
        }
    }
    println!("{}", render_bi2_four_branch_finales_v1(&bundle.index));
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("bi2-four-branch-finales-v1".to_owned())
        .stack_size(1024 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("BI-2 four-branch finale worker panicked".into()),
    }
}
