use google::firestore::v1::Value;

use crate::{
    ser::Error,
    typ::{my_lat_lng::MyLatLng, my_timestamp::MyTimestamp},
};

use super::{
    firestore_geo_point_value_serializer::FirestoreGeoPointValueSerializer,
    firestore_map_value_serializer::FirestoreMapValueSerializer,
    firestore_timestamp_value_serializer::FirestoreTimestampValueSerializer,
};

pub(crate) enum FirestoreValueStructSerializer {
    GeoPoint(FirestoreGeoPointValueSerializer),
    Map(FirestoreMapValueSerializer),
    Timestamp(FirestoreTimestampValueSerializer),
}

impl FirestoreValueStructSerializer {
    pub(crate) fn new(name: &'static str, len: usize) -> Self {
        if name == MyLatLng::NAME {
            FirestoreValueStructSerializer::GeoPoint(FirestoreGeoPointValueSerializer::new())
        } else if name == MyTimestamp::NAME {
            FirestoreValueStructSerializer::Timestamp(FirestoreTimestampValueSerializer::new())
        } else {
            FirestoreValueStructSerializer::Map(FirestoreMapValueSerializer::new(Some(len)))
        }
    }
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
