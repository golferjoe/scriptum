use std::fmt::Display;

use clap::{error::ErrorKind, CommandFactory, FromArgMatches, Parser};
use clap_help::Printer;
use colored::Colorize;

#[derive(Debug, Parser)]
#[command(version)]
pub struct CliArgs {
    /// Source file path
    #[arg(short, long)]
    pub source: String,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<String>,
}

pub fn parse_args() -> CliArgs {
    let cmd = CliArgs::command();
    let matches = cmd.try_get_matches();

    match matches {
        Ok(m) => {
            CliArgs::from_arg_matches(&m).unwrap()
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::DisplayHelp => print_help(),
                ErrorKind::DisplayVersion => print_version(),
                _ => ()
            }

            std::process::exit(0);
        }
    }
}

static INTRO: &str = "

I need to write something here
";

fn print_help() {
    Printer::new(CliArgs::command())
        .with("introduction", INTRO)
        .without("author")
        .print_help();
}

fn print_version() {
    todo!()
}

pub fn comp_msg<S>(text: S) where S: ToString + Display {
    println!("{} {text}", "==>".purple());
}

pub fn err_msg(text: &str) {
    println!(
        "{} {}\n{}",
        "=> Compilation Error:\n==>".red(),
        text.red(),
        "=> Aborting compilation.".red(),
    );
}
