use pen_search::bi4_cone_assembly_v2::{
    emit_bi4_cone_assembly_v2_create_new, issue_bi4_cone_assembly_v2, render_bi4_cone_assembly_v2,
    replay_bi4_cone_assembly_v2,
};
use std::path::Path;

fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let (certificate, emitted) = match arguments.as_slice() {
        [] => (
            issue_bi4_cone_assembly_v2().map_err(|error| error.to_string())?,
            false,
        ),
        [flag, directory] if flag == "--emit" => (
            emit_bi4_cone_assembly_v2_create_new(Path::new(directory))
                .map_err(|error| error.to_string())?,
            true,
        ),
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example bi4_cone_assembly_v2 [--emit <directory>]"
                    .to_owned(),
            );
        }
    };
    if !emitted {
        let replay = replay_bi4_cone_assembly_v2(&certificate);
        if !replay.valid {
            return Err(format!(
                "BI-4 v2 replay failed: {}",
                replay.errors.join("; ")
            ));
        }
    }
    println!("{}", render_bi4_cone_assembly_v2(&certificate));
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("bi4-cone-assembly-v2".to_owned())
        .stack_size(1024 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("BI-4 v2 worker panicked".into()),
    }
}
