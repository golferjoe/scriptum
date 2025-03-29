use regex::Regex;

pub fn minify_html(input: &str) -> String {
    let re_comments = Regex::new(r"<!--.*?-->").unwrap();
    let re_between_tags = Regex::new(r">\s+<").unwrap();
    let re_spaces_eq = Regex::new(r"\s*=\s*").unwrap();
    let re_multi_space = Regex::new(r"[ \t]{2,}").unwrap();

    let no_comments = re_comments.replace_all(input, "");
    let collapsed = re_between_tags.replace_all(&no_comments, "><");
    let cleaned_eq = re_spaces_eq.replace_all(&collapsed, "=");
    let final_result = re_multi_space.replace_all(&cleaned_eq, " ");

    final_result.trim().to_string()
}

pub fn minify_css(input: &str) -> String {
    let re_comments = Regex::new(r"(?s)/\*[^!].*?\*/").unwrap();
    let re_whitespace = Regex::new(r"\s+").unwrap();
    let re_space_around = Regex::new(r"\s*([{}:;,])\s*").unwrap();
    let re_final_semicolon = Regex::new(r";}").unwrap();

    let no_comments = re_comments.replace_all(input, "");
    let collapsed = re_whitespace.replace_all(&no_comments, " ");
    let tightened = re_space_around.replace_all(&collapsed, "$1");
    let cleaned = re_final_semicolon.replace_all(&tightened, "}");

    cleaned.trim().to_string()
}
