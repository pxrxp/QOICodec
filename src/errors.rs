#[derive(Debug)]
pub enum QOIError {
    FileReadError,
    FileWriteError,
    ImageDecodeError(String),
    InvalidArgs(String),
}
