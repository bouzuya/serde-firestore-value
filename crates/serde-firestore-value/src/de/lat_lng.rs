use google::{firestore::v1::Value, r#type::LatLng};
use serde::{
    de::value::{F64Deserializer, StrDeserializer},
    Deserialize,
};

use super::{value_ext::ValueExt, Error};

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "$__serde-firestore-value_private_lat_lng")]
struct MyLatLng {
    latitude: f64,
    longitude: f64,
}

impl From<MyLatLng> for LatLng {
    fn from(
        MyLatLng {
            latitude,
            longitude,
        }: MyLatLng,
    ) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

pub(super) struct FirestoreLatLngValueDeserializer<'de> {
    index: usize,
    lat_lng: &'de LatLng,
}

impl<'de> FirestoreLatLngValueDeserializer<'de> {
    pub(super) fn new(value: &'de Value) -> Result<Self, Error> {
        let lat_lng = value.as_lat_lng()?;
        Ok(Self { index: 0, lat_lng })
    }
}

impl<'de> serde::de::MapAccess<'de> for FirestoreLatLngValueDeserializer<'de> {
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

pub fn deserialize_lat_lng<'de, D>(deserializer: D) -> Result<LatLng, D::Error>
where
    D: serde::Deserializer<'de>,
{
    MyLatLng::deserialize(deserializer).map(LatLng::from)
}

pub fn deserialize_option_lat_lng<'de, D>(deserializer: D) -> Result<Option<LatLng>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<MyLatLng>::deserialize(deserializer).map(|o| o.map(LatLng::from))
}
