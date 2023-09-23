use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, Value};
use prost_types::Timestamp;

use super::{error::ErrorCode, firestore_value_serializer::FirestoreValueSerializer};

use crate::{ser::Error, value_ext::ValueExt};

pub(crate) struct FirestoreTimestampValueSerializer {
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

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        if key == "seconds" {
            let value = value.serialize(FirestoreValueSerializer)?;
            let value = match value.value_type.as_ref() {
                None => todo!(),
                Some(ValueType::IntegerValue(value)) => Ok(*value),
                Some(_) => Err(Self::Error::from(ErrorCode::Custom("TODO".to_string()))),
            }?;
            self.seconds = Some(value);
        } else if key == "nanos" {
            let value = value.serialize(FirestoreValueSerializer)?;
            let value = match value.value_type.as_ref() {
                None => todo!(),
                Some(ValueType::IntegerValue(value)) => Ok(*value),
                Some(_) => Err(Self::Error::from(ErrorCode::Custom("TODO".to_string()))),
            }?;
            let value = i32::try_from(value)
                .map_err(|_| Self::Error::from(ErrorCode::Custom("TODO".to_string())))?;
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
            (Some(seconds), Some(nanos)) => Ok(Timestamp { seconds, nanos }),
        }?;
        Ok(match None::<&'static str> {
            Some(name) => Value::from_fields({
                let mut fields = HashMap::new();
                fields.insert(name.to_string(), Value::from_timestamp(timestamp));
                fields
            }),
            None => Value::from_timestamp(timestamp),
        })
    }
}
