crate::de::private::enum_fields!(Marker, Seconds, Nanos);

pub(crate) struct ProstTypesTimestampMapAccess<'de> {
    iter: std::slice::Iter<'de, Fields>,
    next: Option<&'de Fields>,
    value: &'de prost_types::Timestamp,
}

impl<'de> ProstTypesTimestampMapAccess<'de> {
    pub(crate) fn new(value: &'de prost_types::Timestamp) -> Self {
        Self {
            iter: Fields::VALUES.iter(),
            next: None,
            value,
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for ProstTypesTimestampMapAccess<'de> {
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
                    Fields::Marker => crate::Timestamp::NAME,
                    Fields::Seconds => "seconds",
                    Fields::Nanos => "nanos",
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
                Fields::Seconds => {
                    seed.deserialize(serde::de::value::I64Deserializer::new(self.value.seconds))
                }
                Fields::Nanos => {
                    seed.deserialize(serde::de::value::I32Deserializer::new(self.value.nanos))
                }
            },
        }
    }
}
