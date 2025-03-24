pub type CompilerResult<T> = Result<T, CompilerError>;

#[derive(Debug, thiserror::Error)]
pub enum CompilerError {
    // I/O
    #[error("Failed to open source file: {0}")]
    FileOpen(#[source] std::io::Error),
    #[error("Failed to read a line from source file: {0}")]
    ReadLine(#[source] std::io::Error),

    // Parsing
    #[error("Missing tag closing bracket on line {0}!")]
    MissingClosingBracket(usize),
    #[error("Empty tag on line {0}!")]
    EmptyTag(usize),
    #[error("Invalid tag at line {0}!")]
    InvalidTag(usize),
    // - Heading
    #[error("Invalid heading size property at line {0}!")]
    InvalidHeadingSize(usize),
    // - Image
    #[error("Failed to open attached image '{0}' at line {1}!")]
    InvalidImage(String, usize),
    #[error("Failed to read image '{0}' data: {1}")]
    ImageData(String, #[source] std::io::Error),
}
