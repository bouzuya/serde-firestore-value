use google::firestore::v1::{MapValue, Value};
use serde::de::value::StrDeserializer;

use super::{error::Error, value_ext::ValueExt, FirestoreValueDeserializer};

pub(super) struct FirestoreMapValueDeserializer<'de> {
    iter: std::collections::hash_map::Iter<'de, String, Value>,
    next_value: Option<&'de Value>,
}

impl<'de> FirestoreMapValueDeserializer<'de> {
    pub(super) fn new(value: &'de Value) -> Result<Self, Error> {
        let MapValue { fields } = value.as_map()?;
        Ok(Self {
            iter: fields.iter(),
            next_value: None,
        })
    }
}

impl<'de> serde::de::MapAccess<'de> for FirestoreMapValueDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                if self.next_value.is_none() {
                    self.next_value = Some(value);
                    seed.deserialize(StrDeserializer::new(key)).map(Some)
                } else {
                    unreachable!()
                }
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if let Some(value) = self.next_value.take() {
            seed.deserialize(FirestoreValueDeserializer::new(value))
        } else {
            unreachable!()
        }
    }
}
