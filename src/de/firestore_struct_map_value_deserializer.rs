#[cfg(feature = "btree-map")]
use std::collections::BTreeMap;
#[cfg(feature = "hash-map")]
use std::collections::HashMap;

use serde::de::value::{StrDeserializer, UnitDeserializer};

use crate::google::firestore::v1::Value;
use crate::{Error, value_ext::ValueExt};

use super::FirestoreValueDeserializer;

pub(super) struct FirestoreStructMapValueDeserializer<'de> {
    fields: &'static [&'static str],
    index: usize,
    next_value: Option<&'de Value>,
    #[cfg(feature = "btree-map")]
    values: &'de BTreeMap<String, Value>,
    #[cfg(feature = "hash-map")]
    values: &'de HashMap<String, Value>,
}

impl<'de> FirestoreStructMapValueDeserializer<'de> {
    pub(super) fn new(value: &'de Value, fields: &'static [&'static str]) -> Result<Self, Error> {
        Ok(Self {
            fields,
            index: 0,
            next_value: None,
            values: value.as_fields()?,
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
