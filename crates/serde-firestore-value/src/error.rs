use google_api_proto::google::firestore::v1::value::ValueType;

use super::{value_type_ext::ValueTypeExt, value_type_name::ValueTypeName};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error {
    #[from]
    code: ErrorCode,
}

impl Error {
    pub(super) fn invalid_value_type(value_type: &ValueType, expected: ValueTypeName) -> Self {
        <Self as serde::de::Error>::invalid_type(
            serde::de::Unexpected::Other(value_type.name().as_str()),
            &expected.as_str(),
        )
    }
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::from(ErrorCode::Custom(msg.to_string()))
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::from(ErrorCode::Custom(msg.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ErrorCode {
    #[error("{0}")]
    Custom(String),
    #[error("i16 out of range")]
    I16OutOfRange,
    #[error("i32 out of range")]
    I32OutOfRange,
    #[error("i8 out of range")]
    I8OutOfRange,
    #[error("key must be a string")]
    KeyMustBeAString,
    #[error("maximum byte length (1,048,487 bytes = 1MiB - 89 bytes) exceeded")]
    MaximumByteLengthExceeded,
    #[error("reference value must be a string")]
    ReferenceValueMustBeAString,
    #[error("string is empty")]
    StringIsEmpty,
    #[error("too many chars")]
    TooManyChars,
    #[error("u16 out of range")]
    U16OutOfRange,
    #[error("u32 out of range")]
    U32OutOfRange,
    #[error("u64 is not supported")]
    U64IsNotSupported,
    #[error("u8 out of range")]
    U8OutOfRange,
    #[error("value type must be some")]
    ValueTypeMustBeSome,
}
