//! Create-new runner for the versioned TDC cubical Step 5--8 regression.

use anyhow::{Context, Result};
use pen_eval::tdc1_cubical::{
    CubicalRegressionResult, build_cubical_regression, replay_cubical_regression,
};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

fn usage() -> ! {
    eprintln!(
        "usage:\n  tdc1_cubical_regression run <new-result.json>\n  \
         tdc1_cubical_regression replay <result.json>"
    );
    std::process::exit(2)
}

fn read_result(path: &Path) -> Result<CubicalRegressionResult> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("parse {}", path.display()))
}

fn write_new(path: &Path, result: &CubicalRegressionResult) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(result).context("serialize cubical regression")?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .with_context(|| format!("refusing to overwrite {}", path.display()))?;
    file.write_all(&bytes)
        .with_context(|| format!("write {}", path.display()))?;
    file.write_all(b"\n")
        .with_context(|| format!("finish {}", path.display()))?;
    file.sync_all()
        .with_context(|| format!("sync {}", path.display()))?;
    Ok(())
}

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let Some(mode) = args.first().map(String::as_str) else {
        usage();
    };
    if args.len() != 2 {
        usage();
    }
    let path = PathBuf::from(&args[1]);
    match mode {
        "run" => {
            let result = build_cubical_regression().context("build cubical regression")?;
            write_new(&path, &result)?;
            println!(
                "wrote {} ({}) conditional_registered_basis_counts=2,2,5,10 d4=17 certified_path_nu=undefined full_total=false zone=Z4 d5_attempted=false",
                path.display(),
                result.result_digest
            );
        }
        "replay" => {
            let result = read_result(&path)?;
            replay_cubical_regression(&result).context("replay cubical regression")?;
            println!("replay passed: {}", result.result_digest);
        }
        _ => usage(),
    }
    Ok(())
}
