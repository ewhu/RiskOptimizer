// src/main.rs
/*
 * Main executable for RiskOptimizer
 */

use clap::Parser;
use riskoptimizer::{Result, run};

#[derive(Parser)]
#[command(version, about = RiskOptimizer - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
