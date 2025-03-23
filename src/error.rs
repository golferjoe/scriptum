#[derive(Debug, thiserror::Error)]
pub enum CompilerError {
    #[error("Failed to open source file: {0}")]
    FileOpen(#[source] std::io::Error),
    #[error("Failed to read a line from source file: {0}")]
    ReadLine(#[source] std::io::Error),
}
