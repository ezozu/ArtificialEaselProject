// src/main.rs
/*
 * Main executable for ArtificialEaselProject
 */

use clap::Parser;
use artificialeaselproject::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialEaselProject - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
