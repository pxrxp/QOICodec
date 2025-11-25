#[allow(dead_code)]
#[derive(Debug)]
pub enum QOIError {
    FileReadError,
    FileWriteError,
    ImageDecodeError(String),
    InvalidArgs(String),
    ImageTooLarge,
}
