use crate::google::firestore::v1::Value;
use crate::{Error, value_ext::ValueExt};

use super::private::StageDeserializer;

pub(super) struct FirestorePipelineValueDeserializer<'de> {
    index: usize,
    pipeline: &'de crate::google::firestore::v1::Pipeline,
}

impl<'de> FirestorePipelineValueDeserializer<'de> {
    pub(super) fn new(value: &'de Value) -> Result<Self, Error> {
        let pipeline = value.as_pipeline()?;
        Ok(Self { index: 0, pipeline })
    }
}

impl<'de> serde::de::MapAccess<'de> for FirestorePipelineValueDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index >= 2 {
            Ok(None)
        } else {
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
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.index {
            1 => seed.deserialize(serde::de::value::UnitDeserializer::new()),
            2 => seed.deserialize(serde::de::value::SeqDeserializer::new(
                self.pipeline.stages.iter().map(StageDeserializer),
            )),
            _ => unreachable!(),
        }
    }
}
