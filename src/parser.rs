use std::{fs::File, io::Read};

use base64::{prelude::BASE64_STANDARD, Engine};

use crate::error::{CompilerError, CompilerResult};

// TODO: this code needs to be cleaned a lot

fn one_prop_tag(tag: &str, line_number: usize) -> CompilerResult<(String, String)> {
    let (open, close) = match tag {
        "b" => ("<b>", "</b>"),
        "i" => ("<i>", "</i>"),
        _ => return Err(CompilerError::InvalidTag(line_number))
    };

    Ok((
        open.to_string(),
        close.to_string(),
    ))
}

fn two_props_tag(tag: &[&str], line_number: usize) -> CompilerResult<(String, String)> {
    let tag_name = tag[0];
    let tag_prop = tag[1];

    let tags = match tag_name {
        "h" => {
            // determine size from prop
            let tag_size = match tag_prop {
                "lg" | "lar" | "large" => 1,
                "md" | "med" | "medium" => 2,
                "sm" | "sma" | "small" => 3,
                _ => return Err(CompilerError::InvalidHeadingSize(line_number))
            };

            let html_tag = format!("h{tag_size}");
            (
                format!("<{html_tag}>"),
                format!("</{html_tag}>"),
            )
        }
        "img" => {
            // check if image exists
            let mut image = File::open(tag_prop)
                .map_err(|_| CompilerError::InvalidImage(tag_prop.to_string(), line_number))?;

            // read image data
            let mut image_data = Vec::new(); 
            image.read_to_end(&mut image_data)
                .map_err(|why| CompilerError::ImageData(tag_prop.to_string(), why))?;

            // determine file mime type
            let mime = mime_guess::from_path(tag_prop).first_or_octet_stream();
            println!("mime: {mime:?}");

            // convert to base64
            let base64_data = BASE64_STANDARD.encode(image_data);

            // embed into tag
            (
                format!(r#"<img src="data:{mime:?};base64,{base64_data}"/>"#),
                String::new(),
            )
        }
        _ => return Err(CompilerError::InvalidTag(line_number))
    };

    Ok(tags)
}

fn parse_tag(tag: &[&str], props_count: usize, line_number: usize) -> CompilerResult<(String, String)> {
    // parsing tags based on properties count
    // so we don't have to check if all props were provided for each case
    
    let tags = match props_count {
        1 => one_prop_tag(tag[0], line_number)?,
        2 => two_props_tag(tag, line_number)?,
        _ => return Err(CompilerError::InvalidTag(line_number))
    };

    Ok(tags)
}

pub fn parse_line(line: &str, line_number: usize) -> Result<String, CompilerError> {
    // TODO: test with multi-byte characters

    // check if line starts with tag, otherwise return plain text
    if !line.starts_with('[') {
        return Ok(line.to_string());
    }

    // get closing bracket position, will be used for text extraction
    let closing_bracket = line
        .find(']')
        .ok_or( CompilerError::MissingClosingBracket(line_number))?;

    // extract text after closing bracket
    let text = line[closing_bracket+1..].trim();

    // extract text inside brackets
    let tag = &line[1..closing_bracket].split(':').collect::<Vec<_>>();

    if tag.is_empty() {
        return Err(CompilerError::EmptyTag(line_number));
    }

    // parse tag to html
    let (open_tag, close_tag) = parse_tag(tag, tag.len(), line_number)?;

    Ok(format!("{open_tag}{text}{close_tag}"))
}
