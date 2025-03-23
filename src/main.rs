mod error;
mod parser;

use std::{fs::File, io::{BufRead, BufReader}};

use clap::Parser;
use error::CompilerError;
use parser::parse_line;

#[derive(Debug, Parser)]
struct Args {
    /// Source file path
    #[arg(short, long)]
    source: String,

    /// Output file path
    #[arg(short, long, default_value = "doc.html")]
    output: String,
}

fn main() -> Result<(), CompilerError> {
    let args = Args::parse();

    let source = File::open(&args.source).map_err(CompilerError::FileOpen)?;

    let reader = BufReader::new(source);
    for line in reader.lines() {
        let line = line.map_err(CompilerError::ReadLine)?;
        let _parsed_line = parse_line(&line)?;
    }

    // TODO: store html in a variable and write it to file after parsing

    println!("Compilation finished!");

    Ok(())
}
