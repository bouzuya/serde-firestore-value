use prost_types::Timestamp;
use serde::de::{
    value::{I64Deserializer, StrDeserializer},
    MapAccess,
};

use super::Error;

#[derive(Debug, serde::Deserialize)]
#[serde(
    remote = "Timestamp",
    rename = "$__serde-firestore-value_private_timestamp"
)]
struct MyTimestamp {
    seconds: i64,
    nanos: i32,
}

pub(crate) struct FirestoreTimestampValueDeserializer<'de> {
    pub(crate) index: usize,
    pub(crate) timestamp: &'de Timestamp,
}

impl<'de> MapAccess<'de> for FirestoreTimestampValueDeserializer<'de> {
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
                1 => seed.deserialize(StrDeserializer::new("seconds")).map(Some),
                2 => seed.deserialize(StrDeserializer::new("nanos")).map(Some),
                _ => unreachable!(),
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.index {
            1 => seed.deserialize(I64Deserializer::new(self.timestamp.seconds)),
            2 => seed.deserialize(I64Deserializer::new(i64::from(self.timestamp.nanos))),
            _ => unreachable!(),
        }
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
where
    D: serde::Deserializer<'de>,
{
    MyTimestamp::deserialize(deserializer)
}
