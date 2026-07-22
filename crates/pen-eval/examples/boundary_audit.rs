use pen_eval::boundary_audit::{BoundaryAuditResult, build_boundary_audit, replay_boundary_audit};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

fn read(path: &Path) -> Result<BoundaryAuditResult, Box<dyn std::error::Error>> {
    Ok(serde_json::from_reader(BufReader::new(File::open(path)?))?)
}

fn write_new(path: &Path, result: &BoundaryAuditResult) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new().write(true).create_new(true).open(path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, result)?;
    writer.write_all(b"\n")?;
    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let command = args
        .next()
        .ok_or("usage: boundary_audit <emit|replay> <path>")?;
    let path = args
        .next()
        .ok_or("usage: boundary_audit <emit|replay> <path>")?;
    if args.next().is_some() {
        return Err("usage: boundary_audit <emit|replay> <path>".into());
    }
    let path = Path::new(&path);
    match command.as_str() {
        "emit" => {
            let result = build_boundary_audit()?;
            write_new(path, &result)?;
            println!("{}", result.result_digest);
        }
        "replay" => {
            let result = read(path)?;
            replay_boundary_audit(&result)?;
            println!("{}", result.result_digest);
        }
        _ => return Err("usage: boundary_audit <emit|replay> <path>".into()),
    }
    Ok(())
}
