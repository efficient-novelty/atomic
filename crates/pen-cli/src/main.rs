mod claim_evidence;
mod cli;
mod cmd_benchmark_claim_lane;
mod cmd_certify_claim_lane;
mod cmd_compare_claim_lane;
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
        Commands::CompareClaimLane(args) => cmd_compare_claim_lane::compare_claim_lane(args),
        Commands::CertifyClaimLane(args) => cmd_certify_claim_lane::certify_claim_lane(args),
        Commands::BenchmarkClaimLane(args) => cmd_benchmark_claim_lane::benchmark_claim_lane(args),
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
