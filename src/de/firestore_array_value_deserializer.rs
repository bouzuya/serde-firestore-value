use std::slice::Iter;

use google_api_proto::google::firestore::v1::Value;

use crate::{value_ext::ValueExt, Error};

use super::FirestoreValueDeserializer;

pub(super) struct FirestoreArrayValueDeserializer<'de> {
    iter: Iter<'de, Value>,
}

impl<'de> FirestoreArrayValueDeserializer<'de> {
    pub(super) fn new(value: &'de Value) -> Result<Self, Error> {
        Ok(Self {
            iter: value.as_values()?.iter(),
        })
    }
}

impl<'de> serde::de::SeqAccess<'de> for FirestoreArrayValueDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        self.iter
            .next()
            .map(|value| seed.deserialize(FirestoreValueDeserializer::new(value)))
            .transpose()
    }
}
