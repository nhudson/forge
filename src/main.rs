mod cli;
mod converter;
mod error;
mod openssl;
mod output;

use clap::Parser;
use cli::Args;
use std::process;

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    converter::convert_pfx_to_pem(args)
}
