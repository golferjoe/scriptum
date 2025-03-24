mod error;
mod parser;

use std::{fs::File, io::{BufRead, BufReader}};

use clap::Parser;
use error::{CompilerError, CompilerResult};
use parser::parse_line;

// TODO: colored output

#[derive(Debug, Parser)]
struct Args {
    /// Source file path
    #[arg(short, long)]
    source: String,

    /// Output file path
    #[arg(short, long, default_value = "doc.html")]
    output: String,
}

fn compile(args: &Args) -> CompilerResult<()> {
    let source = File::open(&args.source)
        .map_err(CompilerError::FileOpen)?;

    let reader = BufReader::new(source);
    for (line_number, line) in reader.lines().enumerate() {
        let line = line.map_err(CompilerError::ReadLine)?;
        let parsed_line = parse_line(&line, line_number + 1)?;
        println!("{parsed_line}");
    }

    // TODO: store html in a variable and write it to file after parsing

    Ok(())
}

fn main() {
    let args = Args::parse();

    println!("Current working directory: {:?}", std::env::current_dir());
    println!("Starting compilation...");

    if let Err(why) = compile(&args) {
        eprintln!("[ERROR] {why}");
        eprintln!("Aborting compilation.")
    } else {
        println!("Compilation finished!");
    }
}
