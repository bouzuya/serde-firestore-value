use crate::serializer::Error;

use super::{
    firestore_map_value_serializer::FirestoreMapValueSerializer,
    firestore_timestamp_value_serializer::FirestoreTimestampValueSerializer,
    firestore_value_serializer::FirestoreValueSerializer,
};

pub(crate) enum FirestoreValueStructSerializer<'a> {
    MapValue(FirestoreMapValueSerializer<'a>),
    Timestamp(FirestoreTimestampValueSerializer<'a>),
}

impl<'a> serde::ser::SerializeStruct for FirestoreValueStructSerializer<'a> {
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
        match self {
            Self::MapValue(map_value) => {
                serde::ser::SerializeStruct::serialize_field(map_value, key, value)
            }
            Self::Timestamp(timestamp) => {
                serde::ser::SerializeStruct::serialize_field(timestamp, key, value)
            }
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Self::MapValue(map_value) => serde::ser::SerializeStruct::end(map_value),
            Self::Timestamp(timestamp) => serde::ser::SerializeStruct::end(timestamp),
        }
    }
}
