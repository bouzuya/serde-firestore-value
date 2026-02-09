use crate::Deserializer;

pub(crate) struct GoogleFirestoreFunctionMapAccess<'a> {
    function: &'a crate::google::firestore::v1::Function,
    index: usize,
}

impl<'de> GoogleFirestoreFunctionMapAccess<'de> {
    pub(crate) fn new(function: &'de crate::google::firestore::v1::Function) -> Self {
        Self { function, index: 0 }
    }
}

impl<'de> serde::de::MapAccess<'de> for GoogleFirestoreFunctionMapAccess<'de> {
    type Error = crate::de::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index >= 4 {
            return Ok(None);
        }
        self.index += 1;
        match self.index {
            1 => seed
                .deserialize(serde::de::value::StrDeserializer::new(
                    crate::Function::NAME,
                ))
                .map(Some),
            2 => seed
                .deserialize(serde::de::value::StrDeserializer::new("name"))
                .map(Some),
            3 => seed
                .deserialize(serde::de::value::StrDeserializer::new("args"))
                .map(Some),
            4 => seed
                .deserialize(serde::de::value::StrDeserializer::new("options"))
                .map(Some),
            _ => unreachable!(),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.index {
            1 => seed.deserialize(serde::de::value::UnitDeserializer::new()),
            2 => seed.deserialize(serde::de::value::StrDeserializer::new(&self.function.name)),
            3 => seed.deserialize(serde::de::value::SeqDeserializer::new(
                self.function.args.iter().map(Deserializer::new),
            )),
            4 => seed.deserialize(serde::de::value::MapDeserializer::new(
                self.function
                    .options
                    .iter()
                    .map(|(k, v)| (k.as_str(), Deserializer::new(v))),
            )),
            _ => unreachable!(),
        }
    }
}
