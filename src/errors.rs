#[derive(Debug)]
pub enum QOIError {
    FileReadError,
    FileWriteError,
    ImageDecodeError,
    InvalidArgs,
}
