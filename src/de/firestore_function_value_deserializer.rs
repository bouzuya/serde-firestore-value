use super::private::ValueDeserializer;
use crate::google::firestore::v1::Value;
use crate::{Error, value_ext::ValueExt};

pub(super) struct FirestoreFunctionValueDeserializer<'de> {
    index: usize,
    function: &'de crate::google::firestore::v1::Function,
}

impl<'de> FirestoreFunctionValueDeserializer<'de> {
    pub(super) fn new(value: &'de Value) -> Result<Self, Error> {
        let function = value.as_function()?;
        Ok(Self { index: 0, function })
    }
}

impl<'de> serde::de::MapAccess<'de> for FirestoreFunctionValueDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index >= 3 {
            Ok(None)
        } else {
            self.index += 1;
            match self.index {
                1 => seed
                    .deserialize(serde::de::value::StrDeserializer::new("name"))
                    .map(Some),
                2 => seed
                    .deserialize(serde::de::value::StrDeserializer::new("args"))
                    .map(Some),
                3 => seed
                    .deserialize(serde::de::value::StrDeserializer::new("options"))
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
            1 => seed.deserialize(serde::de::value::StrDeserializer::new(&self.function.name)),
            2 => seed.deserialize(serde::de::value::SeqDeserializer::new(
                self.function.args.iter().map(ValueDeserializer),
            )),
            3 => seed.deserialize(serde::de::value::MapDeserializer::new(
                self.function
                    .options
                    .iter()
                    .map(|(k, v)| (k.as_str(), ValueDeserializer(v))),
            )),
            _ => unreachable!(),
        }
    }
}
