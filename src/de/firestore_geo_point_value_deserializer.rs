use serde::de::value::{F64Deserializer, StrDeserializer};

use crate::google::{firestore::v1::Value, r#type::LatLng as GoogleApiProtoLatLng};
use crate::value_ext::ValueExt;

use super::Error;

pub(super) struct FirestoreGeoPointValueDeserializer<'de> {
    index: usize,
    lat_lng: &'de GoogleApiProtoLatLng,
}

impl<'de> FirestoreGeoPointValueDeserializer<'de> {
    pub(super) fn new(value: &'de Value) -> Result<Self, Error> {
        let lat_lng = value.as_lat_lng()?;
        Ok(Self { index: 0, lat_lng })
    }
}

impl<'de> serde::de::MapAccess<'de> for FirestoreGeoPointValueDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index >= 2 {
            Ok(None)
        } else {
            self.index += 1;
            match self.index {
                1 => seed.deserialize(StrDeserializer::new("latitude")).map(Some),
                2 => seed
                    .deserialize(StrDeserializer::new("longitude"))
                    .map(Some),
                _ => unreachable!(),
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.index {
            1 => seed.deserialize(F64Deserializer::new(self.lat_lng.latitude)),
            2 => seed.deserialize(F64Deserializer::new(self.lat_lng.longitude)),
            _ => unreachable!(),
        }
    }
}
