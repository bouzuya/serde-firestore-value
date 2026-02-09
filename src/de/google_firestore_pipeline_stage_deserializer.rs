pub(crate) struct GoogleFirestorePipelineStageDeserializer<'de>(
    &'de crate::google::firestore::v1::pipeline::Stage,
);

impl<'de> GoogleFirestorePipelineStageDeserializer<'de> {
    pub(crate) fn new(stage: &'de crate::google::firestore::v1::pipeline::Stage) -> Self {
        Self(stage)
    }
}

impl<'de> serde::Deserializer<'de> for GoogleFirestorePipelineStageDeserializer<'de> {
    type Error = crate::de::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(crate::de::GoogleFirestorePipelineStageMapAccess::new(
            self.0,
        ))
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> serde::de::IntoDeserializer<'de, crate::de::Error>
    for GoogleFirestorePipelineStageDeserializer<'de>
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}
