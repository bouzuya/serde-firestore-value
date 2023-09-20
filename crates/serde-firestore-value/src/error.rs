#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error {
    #[from]
    code: ErrorCode,
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error {
            code: ErrorCode::Custom(msg.to_string()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ErrorCode {
    #[error("{0}")]
    Custom(String),
    #[error("key must be a string")]
    KeyMustBeAString,
    #[error("maximum byte length (1,048,487 bytes = 1MiB - 89 bytes) exceeded")]
    MaximumByteLengthExceeded,
    #[error("u64 is not supported")]
    U64IsNotSupported,
}
