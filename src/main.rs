mod error;
mod cli;
mod parser;

use std::{fs::File, io::{BufRead, BufReader, BufWriter, Write}, path::Path};

use clap::crate_version;
use cli::{comp_msg, err_msg, parse_args, CliArgs};
use error::{CompilerError, CompilerResult};
use parser::parse_line;

// Returns output file name or fallback when it's not specified by user
fn output_file_name(args: &CliArgs) -> String {
    match args.output.clone() {
        Some(o) => o,
        None => {
            let file_name = Path::new(&args.source)
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
                .unwrap_or("doc.html".to_string());
            format!("{file_name}.html")
        }
    }
}

fn compile(args: &CliArgs) -> CompilerResult<()> {
    let source = File::open(&args.source)
        .map_err(CompilerError::FileOpen)?;

    let reader = BufReader::new(source);
    let mut parsed_lines = Vec::new();

    comp_msg("Parsing file...");
    for (line_number, line) in reader.lines().enumerate() {
        let line = line.map_err(CompilerError::ReadLine)?;
        let parsed_line = parse_line(&line, line_number + 1)?;
        parsed_lines.push(parsed_line);
    }
    comp_msg("File parsed");

    let output_name = output_file_name(args);
    comp_msg(format!("Writing output to '{output_name}'..."));

    let output = File::create(&output_name)
        .map_err(CompilerError::OutputCreate)?;

    let mut writer = BufWriter::new(output);
    for line in parsed_lines {
        writeln!(writer, "{}", line).map_err(CompilerError::OutputWrite)?;
    }

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
