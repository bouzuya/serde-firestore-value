crate::de::private::enum_fields!(Marker, Stages);

pub(crate) struct GoogleFirestorePipelineMapAccess<'de> {
    iter: std::slice::Iter<'de, Fields>,
    next: Option<&'de Fields>,
    value: &'de crate::google::firestore::v1::Pipeline,
}

impl<'de> GoogleFirestorePipelineMapAccess<'de> {
    pub(crate) fn new(value: &'de crate::google::firestore::v1::Pipeline) -> Self {
        Self {
            iter: Fields::VALUES.iter(),
            next: None,
            value,
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for GoogleFirestorePipelineMapAccess<'de> {
    type Error = crate::de::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            None => Ok(None),
            Some(field) => {
                self.next = Some(field);
                seed.deserialize(serde::de::value::StrDeserializer::new(match field {
                    Fields::Marker => crate::Pipeline::NAME,
                    Fields::Stages => "stages",
                }))
                .map(Some)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.next.take() {
            None => panic!("next_value_seed called before next_key_seed"),
            Some(field) => match field {
                Fields::Marker => seed.deserialize(serde::de::value::UnitDeserializer::new()),
                Fields::Stages => seed.deserialize(serde::de::value::SeqDeserializer::new(
                    self.value
                        .stages
                        .iter()
                        .map(crate::de::GoogleFirestorePipelineStageMapAccess::new)
                        .map(serde::de::value::MapAccessDeserializer::new),
                )),
            },
        }
    }
}
