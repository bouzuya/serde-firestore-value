use google::firestore::v1::Value;

use crate::ser::Error;

use super::{
    firestore_map_value_serializer::FirestoreMapValueSerializer,
    firestore_timestamp_value_serializer::FirestoreTimestampValueSerializer,
};

pub(crate) enum FirestoreValueStructSerializer {
    MapValue(FirestoreMapValueSerializer),
    Timestamp(FirestoreTimestampValueSerializer),
}

impl serde::ser::SerializeStruct for FirestoreValueStructSerializer {
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
