use pen_search::t_bi_nu1_regression::{
    emit_t_bi_nu1_regression_create_new, replay_t_bi_nu1_regression_directory,
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, directory] if command == "create-new" => {
            let certificate = emit_t_bi_nu1_regression_create_new(Path::new(directory))?;
            println!("{}", serde_json::to_string_pretty(&certificate)?);
        }
        [command, directory] if command == "replay" => {
            let replay = replay_t_bi_nu1_regression_directory(Path::new(directory))?;
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err("T-BI-NU1 replay failed".into());
            }
        }
        _ => {
            return Err("usage: t_bi_nu1_regression <create-new|replay> <output-directory>".into());
        }
    }
    Ok(())
}
