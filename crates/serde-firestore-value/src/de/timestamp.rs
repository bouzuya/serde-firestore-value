use google::firestore::v1::Value;
use prost_types::Timestamp;
use serde::de::value::{I64Deserializer, StrDeserializer};

use super::{value_ext::ValueExt, Error};

#[derive(Debug, serde::Deserialize)]
#[serde(
    remote = "Timestamp",
    rename = "$__serde-firestore-value_private_timestamp"
)]
struct MyTimestamp {
    seconds: i64,
    nanos: i32,
}

pub(super) struct FirestoreTimestampValueDeserializer<'de> {
    index: usize,
    timestamp: &'de Timestamp,
}

impl<'de> FirestoreTimestampValueDeserializer<'de> {
    pub(super) fn new(value: &'de Value) -> Result<Self, Error> {
        let timestamp = value.as_timestamp()?;
        Ok(Self {
            index: 0,
            timestamp,
        })
    }
}

impl<'de> serde::de::MapAccess<'de> for FirestoreTimestampValueDeserializer<'de> {
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
