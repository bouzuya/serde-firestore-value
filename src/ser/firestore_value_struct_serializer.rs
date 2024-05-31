use google_api_proto::google::firestore::v1::Value;

use crate::{ser::Error, LatLng, Timestamp};

use super::{
    firestore_geo_point_value_serializer::FirestoreGeoPointValueSerializer,
    firestore_map_value_serializer::FirestoreMapValueSerializer,
    firestore_timestamp_value_serializer::FirestoreTimestampValueSerializer,
};

#[doc(hidden)]
pub enum FirestoreValueStructSerializer {
    GeoPoint(FirestoreGeoPointValueSerializer),
    Map(FirestoreMapValueSerializer),
    Timestamp(FirestoreTimestampValueSerializer),
}

impl FirestoreValueStructSerializer {
    pub(crate) fn new(name: &'static str, len: usize) -> Self {
        if name == LatLng::NAME {
            FirestoreValueStructSerializer::GeoPoint(FirestoreGeoPointValueSerializer::new())
        } else if name == Timestamp::NAME {
            FirestoreValueStructSerializer::Timestamp(FirestoreTimestampValueSerializer::new())
        } else {
            FirestoreValueStructSerializer::Map(FirestoreMapValueSerializer::new(Some(len)))
        }
    }
}

impl serde::ser::SerializeStruct for FirestoreValueStructSerializer {
    type Ok = Value;

    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        match self {
            Self::GeoPoint(s) => serde::ser::SerializeStruct::serialize_field(s, key, value),
            Self::Map(s) => serde::ser::SerializeStruct::serialize_field(s, key, value),
            Self::Timestamp(s) => serde::ser::SerializeStruct::serialize_field(s, key, value),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Self::GeoPoint(s) => serde::ser::SerializeStruct::end(s),
            Self::Map(s) => serde::ser::SerializeStruct::end(s),
            Self::Timestamp(s) => serde::ser::SerializeStruct::end(s),
        }
    }
}
