use pen_search::agent_a_hist_cert_v4::{
    emit_agent_a_hist_cert_v4_create_new, replay_agent_a_hist_cert_v4_json,
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let command = args
        .next()
        .ok_or("usage: agent_a_hist_cert_v4 <emit|replay> <path>")?;
    let path = args
        .next()
        .ok_or("usage: agent_a_hist_cert_v4 <emit|replay> <path>")?;
    if args.next().is_some() {
        return Err("usage: agent_a_hist_cert_v4 <emit|replay> <path>".into());
    }
    let replay = match command.as_str() {
        "emit" => emit_agent_a_hist_cert_v4_create_new(Path::new(&path))?,
        "replay" => replay_agent_a_hist_cert_v4_json(&std::fs::read_to_string(path)?),
        _ => return Err("usage: agent_a_hist_cert_v4 <emit|replay> <path>".into()),
    };
    println!("{}", serde_json::to_string_pretty(&replay)?);
    if replay.valid {
        Ok(())
    } else {
        Err("Agent A HIST-CERT v4 replay failed".into())
    }
}
