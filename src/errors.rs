use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
}
