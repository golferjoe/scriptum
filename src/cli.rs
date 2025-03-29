use std::fmt::Display;

use clap::{error::ErrorKind, CommandFactory, FromArgMatches, Parser};
use clap_help::Printer;
use colored::Colorize;
use termimad::{crossterm::style::Color, CompoundStyle};

const INTRO: &str = "

Super-cool CLI tool for compiling Scriptum notes to HTML documents.
";

const HELP_COLOR: Color = Color::Magenta;

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

fn print_help() {
    let mut printer = Printer::new(CliArgs::command())
        .with("introduction", INTRO)
        .with("options", clap_help::TEMPLATE_OPTIONS_MERGED_VALUE)
        .without("author");

    let skin = printer.skin_mut();
    skin.headers[0].compound_style.set_fg(HELP_COLOR);
    skin.bold.set_fg(HELP_COLOR);
    skin.italic = CompoundStyle::with_fg(HELP_COLOR);

    printer.print_help();
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
