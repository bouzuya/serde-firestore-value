use std::collections::HashMap;

use google::firestore::v1::{MapValue, Value};
use serde::de::value::{StrDeserializer, UnitDeserializer};

use super::{error::Error, value_ext::ValueExt, FirestoreValueDeserializer};

pub(super) struct FirestoreStructMapValueDeserializer<'de> {
    fields: &'static [&'static str],
    index: usize,
    next_value: Option<&'de Value>,
    values: &'de HashMap<String, Value>,
}

impl<'de> FirestoreStructMapValueDeserializer<'de> {
    pub(super) fn new(value: &'de Value, fields: &'static [&'static str]) -> Result<Self, Error> {
        let MapValue { fields: values } = value.as_map()?;
        Ok(Self {
            fields,
            index: 0,
            next_value: None,
            values,
        })
    }
}

impl<'de> serde::de::MapAccess<'de> for FirestoreStructMapValueDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index >= self.fields.len() {
            Ok(None)
        } else {
            let field = self.fields[self.index];
            self.index += 1;
            if self.next_value.is_none() {
                self.next_value = self.values.get(field);
                seed.deserialize(StrDeserializer::new(field)).map(Some)
            } else {
                unreachable!()
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if let Some(value) = self.next_value.take() {
            seed.deserialize(FirestoreValueDeserializer::new(value))
        } else {
            seed.deserialize(UnitDeserializer::new())
        }
    }
}
