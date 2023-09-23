use google::firestore::v1::value::ValueType;
use prost_types::Timestamp;

use super::{error::ErrorCode, firestore_value_serializer::FirestoreValueSerializer};

use crate::serializer::Error;

pub(crate) struct FirestoreTimestampValueSerializer<'a> {
    parent: &'a mut FirestoreValueSerializer,
    seconds: Option<i64>,
    nanos: Option<i32>,
}

impl<'a> FirestoreTimestampValueSerializer<'a> {
    pub(crate) const NAME: &str = "$__serde-firestore-value_private_timestamp";

    pub(crate) fn new(parent: &'a mut FirestoreValueSerializer) -> Self {
        Self {
            parent,
            seconds: None,
            nanos: None,
        }
    }
}

impl<'a> serde::ser::SerializeStruct for FirestoreTimestampValueSerializer<'a> {
    type Ok = &'a mut FirestoreValueSerializer;

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
            let mut serializer = FirestoreValueSerializer::default();
            value.serialize(&mut serializer)?;
            let value = serializer.into_inner();
            let value = match value.value_type.as_ref() {
                None => todo!(),
                Some(ValueType::IntegerValue(value)) => Ok(*value),
                Some(_) => Err(Self::Error::from(ErrorCode::Custom("TODO".to_string()))),
            }?;
            self.seconds = Some(value);
        } else if key == "nanos" {
            let mut serializer = FirestoreValueSerializer::default();
            value.serialize(&mut serializer)?;
            let value = serializer.into_inner();
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
        self.parent.set_timestamp_value(None, timestamp);
        Ok(self.parent)
    }
}
