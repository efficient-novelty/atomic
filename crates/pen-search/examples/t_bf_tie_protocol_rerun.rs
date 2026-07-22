use pen_search::t_bf_tie_protocol_rerun::{
    TbfTieProtocolRerunCertificate, emit_t_bf_tie_protocol_rerun_create_new,
    replay_t_bf_tie_protocol_rerun_certificate,
};
use std::path::Path;

fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, path] if command == "create-new" => {
            let replay = emit_t_bf_tie_protocol_rerun_create_new(Path::new(path))
                .map_err(|error| error.to_string())?;
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
        }
        [command, path] if command == "replay" => {
            let certificate: TbfTieProtocolRerunCertificate = serde_json::from_str(
                &std::fs::read_to_string(path).map_err(|error| error.to_string())?,
            )
            .map_err(|error| error.to_string())?;
            let replay = replay_t_bf_tie_protocol_rerun_certificate(&certificate);
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
            if !replay.valid {
                return Err("T-BF1/T-BF3 tie-protocol rerun failed".to_owned());
            }
        }
        _ => {
            return Err(
                "usage: t_bf_tie_protocol_rerun <create-new|replay> <artifact.json>".to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("t-bf-tie-protocol-rerun-driver".to_owned())
        .stack_size(256 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("T-BF1/T-BF3 tie-protocol rerun driver panicked".into()),
    }
}
