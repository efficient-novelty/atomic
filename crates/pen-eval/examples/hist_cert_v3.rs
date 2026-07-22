use pen_eval::tdc1_hist_cert_v3::{hist_cert_v3_json_pretty, replay_hist_cert_v3_json};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let command = args
        .next()
        .ok_or("usage: hist_cert_v3 <emit|replay> <path>")?;
    let path = args
        .next()
        .ok_or("usage: hist_cert_v3 <emit|replay> <path>")?;
    if args.next().is_some() {
        return Err("usage: hist_cert_v3 <emit|replay> <path>".into());
    }
    let path = Path::new(&path);
    match command.as_str() {
        "emit" => {
            let json = hist_cert_v3_json_pretty()?;
            let mut output = OpenOptions::new().write(true).create_new(true).open(path)?;
            output.write_all(json.as_bytes())?;
            output.flush()?;
            let replay = replay_hist_cert_v3_json(&json);
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err(format!("new HIST-CERT v3 failed replay: {:?}", replay.errors).into());
            }
        }
        "replay" => {
            let json = std::fs::read_to_string(path)?;
            let replay = replay_hist_cert_v3_json(&json);
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err("HIST-CERT v3 replay failed".into());
            }
        }
        _ => return Err("usage: hist_cert_v3 <emit|replay> <path>".into()),
    }
    Ok(())
}
