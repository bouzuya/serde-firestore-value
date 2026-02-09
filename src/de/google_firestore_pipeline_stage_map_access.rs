pub(crate) struct GoogleFirestorePipelineStageMapAccess<'de> {
    index: usize,
    stage: &'de crate::google::firestore::v1::pipeline::Stage,
}

impl<'de> GoogleFirestorePipelineStageMapAccess<'de> {
    pub(crate) fn new(stage: &'de crate::google::firestore::v1::pipeline::Stage) -> Self {
        Self { index: 0, stage }
    }
}

impl<'de> serde::de::MapAccess<'de> for GoogleFirestorePipelineStageMapAccess<'de> {
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
                .deserialize(serde::de::value::StrDeserializer::new(crate::Stage::NAME))
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
            2 => seed.deserialize(serde::de::value::StrDeserializer::new(&self.stage.name)),
            3 => seed.deserialize(serde::de::value::SeqDeserializer::new(
                self.stage.args.iter().map(crate::de::Deserializer::new),
            )),
            4 => seed.deserialize(serde::de::value::MapDeserializer::new(
                self.stage
                    .options
                    .iter()
                    .map(|(k, v)| (k.as_str(), crate::de::Deserializer::new(v))),
            )),
            _ => unreachable!(),
        }
    }
}
