use super::firestore_value_serializer::FirestoreValueSerializer;

use crate::google::firestore::v1::Value;
use crate::{error::ErrorCode, value_ext::ValueExt, Error};

#[doc(hidden)]
pub struct FirestoreTimestampValueSerializer {
    seconds: Option<i64>,
    nanos: Option<i32>,
}

impl FirestoreTimestampValueSerializer {
    pub(crate) fn new() -> Self {
        Self {
            seconds: None,
            nanos: None,
        }
    }
}

impl serde::ser::SerializeStruct for FirestoreTimestampValueSerializer {
    type Ok = Value;

    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if key == "seconds" {
            let value = value.serialize(FirestoreValueSerializer)?;
            let value = value.as_integer()?;
            self.seconds = Some(value);
        } else if key == "nanos" {
            let value = value.serialize(FirestoreValueSerializer)?;
            let value = value.as_integer()?;
            let value =
                i32::try_from(value).map_err(|_| Self::Error::from(ErrorCode::I32OutOfRange))?;
            self.nanos = Some(value);
        } else {
            // TODO: invalid timestamp
            todo!()
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let timestamp = match (self.seconds, self.nanos) {
            (None, None) | (None, Some(_)) | (Some(_), None) => {
                Err(Self::Error::from(ErrorCode::Custom("TODO".to_string())))
            }
            (Some(seconds), Some(nanos)) => Ok(prost_types::Timestamp { seconds, nanos }),
        }?;
        Ok(match None::<&'static str> {
            Some(name) => Value::from_fields([(name, Value::from_timestamp(timestamp))]),
            None => Value::from_timestamp(timestamp),
        })
    }
}
