pub(crate) struct ProstTypesTimestampMapAccess<'de> {
    index: usize,
    timestamp: &'de prost_types::Timestamp,
}

impl<'de> ProstTypesTimestampMapAccess<'de> {
    pub(crate) fn new(timestamp: &'de prost_types::Timestamp) -> Self {
        Self {
            index: 0,
            timestamp,
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for ProstTypesTimestampMapAccess<'de> {
    type Error = crate::de::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index >= 3 {
            return Ok(None);
        }
        self.index += 1;
        match self.index {
            1 => seed
                .deserialize(serde::de::value::StrDeserializer::new(
                    crate::Timestamp::NAME,
                ))
                .map(Some),
            2 => seed
                .deserialize(serde::de::value::StrDeserializer::new("seconds"))
                .map(Some),
            3 => seed
                .deserialize(serde::de::value::StrDeserializer::new("nanos"))
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
            2 => seed.deserialize(serde::de::value::I64Deserializer::new(
                self.timestamp.seconds,
            )),
            3 => seed.deserialize(serde::de::value::I32Deserializer::new(self.timestamp.nanos)),
            _ => unreachable!(),
        }
    }
}
