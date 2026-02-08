use crate::google::firestore::v1::Value;
use crate::typ::{Function, Pipeline};
use crate::{LatLng, Timestamp, ser::Error};

use super::{
    firestore_function_value_serializer::FirestoreFunctionValueSerializer,
    firestore_geo_point_value_serializer::FirestoreGeoPointValueSerializer,
    firestore_map_value_serializer::FirestoreMapValueSerializer,
    firestore_pipeline_value_serializer::FirestorePipelineValueSerializer,
    firestore_timestamp_value_serializer::FirestoreTimestampValueSerializer,
};

#[doc(hidden)]
pub enum FirestoreValueStructSerializer {
    Function(FirestoreFunctionValueSerializer),
    GeoPoint(FirestoreGeoPointValueSerializer),
    Map(FirestoreMapValueSerializer),
    Pipeline(FirestorePipelineValueSerializer),
    Timestamp(FirestoreTimestampValueSerializer),
}

impl FirestoreValueStructSerializer {
    pub(crate) fn new(name: &'static str, len: usize) -> Self {
        if name == Function::NAME {
            FirestoreValueStructSerializer::Function(FirestoreFunctionValueSerializer::new())
        } else if name == LatLng::NAME {
            FirestoreValueStructSerializer::GeoPoint(FirestoreGeoPointValueSerializer::new())
        } else if name == Pipeline::NAME {
            FirestoreValueStructSerializer::Pipeline(FirestorePipelineValueSerializer::new())
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
            Self::Function(s) => serde::ser::SerializeStruct::serialize_field(s, key, value),
            Self::GeoPoint(s) => serde::ser::SerializeStruct::serialize_field(s, key, value),
            Self::Map(s) => serde::ser::SerializeStruct::serialize_field(s, key, value),
            Self::Pipeline(s) => serde::ser::SerializeStruct::serialize_field(s, key, value),
            Self::Timestamp(s) => serde::ser::SerializeStruct::serialize_field(s, key, value),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Self::Function(s) => serde::ser::SerializeStruct::end(s),
            Self::GeoPoint(s) => serde::ser::SerializeStruct::end(s),
            Self::Map(s) => serde::ser::SerializeStruct::end(s),
            Self::Pipeline(s) => serde::ser::SerializeStruct::end(s),
            Self::Timestamp(s) => serde::ser::SerializeStruct::end(s),
        }
    }
}
