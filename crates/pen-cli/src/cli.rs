use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "pen-cli", about = "Strict-only PEN bootstrap CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run(RunArgs),
    Resume(ResumeArgs),
    Inspect(InspectArgs),
    ExportAgda(ExportAgdaArgs),
}

#[derive(Debug, Clone, Args)]
pub struct RunArgs {
    #[arg(long, default_value = "configs/default.toml")]
    pub config: PathBuf,
    #[arg(long, default_value = "runs")]
    pub root: PathBuf,
    #[arg(long)]
    pub run_id: Option<String>,
    #[arg(long)]
    pub until_step: Option<u32>,
    #[arg(long)]
    pub debug: bool,
}

#[derive(Debug, Clone, Args)]
pub struct ResumeArgs {
    pub run_dir: PathBuf,
    #[arg(long)]
    pub until_step: Option<u32>,
    #[arg(long)]
    pub debug: bool,
}

#[derive(Debug, Clone, Args)]
pub struct InspectArgs {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Args)]
pub struct ExportAgdaArgs {
    #[arg(long)]
    pub run_dir: Option<PathBuf>,
    #[arg(long)]
    pub output_dir: Option<PathBuf>,
    #[arg(long)]
    pub until_step: Option<u32>,
    #[arg(long)]
    pub verify: bool,
}
