use std::{fs::File, io::{BufWriter, Write}, path::Path};

use crate::{cli::{comp_msg, CliArgs}, error::{CompilerError, CompilerResult}, utils::{minify_html, minify_css}};

const NORMALIZE_CSS: &str = include_str!("styles/normalize.css");
const MAIN_CSS: &str = include_str!("styles/main.css");

const HTML_HEAD: &str = r#"
<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <title>Scriptum</title>
        <style>
"#;

const HTML_BODY: &str = r#"
        </style>
    </head>
    <body>
        <div class="main">
"#;

const HTML_FOOT: &str = r#"
        </div>
    </body>
</html>
"#;

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

// Joins and minifies all HTML head code (i.e. HTML_HEAD, CSS, HTML_BODY)
fn html_head() -> String {
    let mut out = String::new();

    out.push_str(&minify_html(HTML_HEAD));
    out.push_str(&minify_css(NORMALIZE_CSS));
    out.push_str(&minify_css(MAIN_CSS));
    out.push_str(&minify_html(HTML_BODY));

    out
}

fn write_text(writer: &mut BufWriter<File>, text: &str) -> CompilerResult<()> {
    write!(writer, "{}", text).map_err(CompilerError::OutputWrite)
}

pub fn create_output_file(args: &CliArgs, lines: &[String]) -> CompilerResult<()> {
    // figure out file name
    let output_name = output_file_name(args);
    comp_msg(format!("Writing output to '{output_name}'..."));

    // create output file
    let output = File::create(&output_name)
        .map_err(CompilerError::OutputCreate)?;
    let mut writer = BufWriter::new(output);

    // html skeleton
    let head = html_head();
    write_text(&mut writer, &head)?;

    // write parsed lines
    for line in lines {
        write_text(&mut writer, line)?;
    }

    // finish html skeleton
    let foot = minify_html(HTML_FOOT);
    write_text(&mut writer, &foot)?;

    Ok(())
}
