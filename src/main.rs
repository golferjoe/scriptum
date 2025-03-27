mod error;
mod cli;
mod parser;
mod html;

use std::{fs::File, io::{BufRead, BufReader}};

use clap::crate_version;
use cli::{comp_msg, err_msg, parse_args, CliArgs};
use error::{CompilerError, CompilerResult};
use html::create_output_file;
use parser::parse_line;

fn compile(args: &CliArgs) -> CompilerResult<()> {
    let source = File::open(&args.source)
        .map_err(CompilerError::FileOpen)?;

    let reader = BufReader::new(source);
    let mut parsed_lines = Vec::new();

    comp_msg("Parsing file...");
    for (line_number, line) in reader.lines().enumerate() {
        let line = line.map_err(CompilerError::ReadLine)?;
        if line.is_empty() {
            continue;
        }

        let parsed_line = parse_line(&line, line_number + 1)?;
        parsed_lines.push(parsed_line);
    }
    comp_msg("File parsed");

    create_output_file(args, &parsed_lines)?;

    Ok(())
}

fn main() {
    let args = parse_args();

    comp_msg(format!("ANLC v{} (https://github.com/golferjoe/anlc)", crate_version!()));
    comp_msg(format!("Source file: {}", args.source));
    comp_msg("Starting compilation...");

    if let Err(why) = compile(&args) {
        err_msg(&why.to_string());
    } else {
        comp_msg("Compilation finished!");
    }
}
