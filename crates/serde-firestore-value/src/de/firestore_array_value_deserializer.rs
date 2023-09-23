use google::firestore::v1::{ArrayValue, Value};

use super::{error::Error, value_ext::ValueExt, FirestoreValueDeserializer};

pub(super) struct FirestoreArrayValueDeserializer<'de> {
    index: usize,
    value: &'de Value,
}

impl<'de> FirestoreArrayValueDeserializer<'de> {
    pub(super) fn new(value: &'de Value) -> Result<Self, Error> {
        value.as_array()?;
        Ok(Self { index: 0, value })
    }
}

impl<'de> serde::de::SeqAccess<'de> for FirestoreArrayValueDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let ArrayValue { values } = self.value.as_array()?;
        if self.index < values.len() {
            let value = &values[self.index];
            self.index += 1;
            seed.deserialize(FirestoreValueDeserializer { value })
                .map(Some)
        } else {
            Ok(None)
        }
    }
}
