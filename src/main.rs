use std::fs::File;

use anyhow::Context;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// Source file path
    #[arg(short, long)]
    source: String,

    /// Output file path
    #[arg(short, long, default_value = "doc.html")]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let source = File::open(&args.source)
        .with_context(|| format!("Failed to open source file '{}'! Does it exists?", args.source))?;

    let output = File::create(&args.output)
        .context("Failed to create output file! Do I have permissions?")?;

    println!("Compilation finished!");

    Ok(())
}
