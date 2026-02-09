pub(crate) struct GoogleFirestorePipelineMapAccess<'de> {
    index: usize,
    pipeline: &'de crate::google::firestore::v1::Pipeline,
}

impl<'de> GoogleFirestorePipelineMapAccess<'de> {
    pub(crate) fn new(pipeline: &'de crate::google::firestore::v1::Pipeline) -> Self {
        Self { index: 0, pipeline }
    }
}

impl<'de> serde::de::MapAccess<'de> for GoogleFirestorePipelineMapAccess<'de> {
    type Error = crate::de::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index >= 2 {
            return Ok(None);
        }
        self.index += 1;
        match self.index {
            1 => seed
                .deserialize(serde::de::value::StrDeserializer::new(
                    crate::Pipeline::NAME,
                ))
                .map(Some),
            2 => seed
                .deserialize(serde::de::value::StrDeserializer::new("stages"))
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
            2 => seed.deserialize(serde::de::value::SeqDeserializer::new(
                self.pipeline
                    .stages
                    .iter()
                    .map(crate::de::GoogleFirestorePipelineStageDeserializer::new),
            )),
            _ => unreachable!(),
        }
    }
}
