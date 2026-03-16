mod cli;
mod cmd_export_agda;
mod cmd_inspect;
mod cmd_resume;
mod cmd_run;
mod human;
mod narrative;
mod output;
mod progress;
mod report;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Run(args) => cmd_run::run(args),
        Commands::Resume(args) => cmd_resume::resume(args),
        Commands::Inspect(args) => cmd_inspect::inspect(args),
        Commands::ExportAgda(args) => cmd_export_agda::export_agda(args),
    };

    match result {
        Ok(output) => {
            if !output.is_empty() {
                println!("{output}");
            }
        }
        Err(error) => {
            eprintln!("error: {error:#}");
            std::process::exit(1);
        }
    }
}
