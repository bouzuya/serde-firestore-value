crate::de::private::enum_fields!(Marker, Name, Args, Options);

pub(crate) struct GoogleFirestoreFunctionMapAccess<'de> {
    iter: std::slice::Iter<'de, Fields>,
    next: Option<&'de Fields>,
    value: &'de crate::google::firestore::v1::Function,
}

impl<'de> GoogleFirestoreFunctionMapAccess<'de> {
    pub(crate) fn new(value: &'de crate::google::firestore::v1::Function) -> Self {
        Self {
            iter: Fields::VALUES.iter(),
            next: None,
            value,
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for GoogleFirestoreFunctionMapAccess<'de> {
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
                    Fields::Marker => crate::Function::NAME,
                    Fields::Name => "name",
                    Fields::Args => "args",
                    Fields::Options => "options",
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
                Fields::Name => {
                    seed.deserialize(serde::de::value::StrDeserializer::new(&self.value.name))
                }
                Fields::Args => seed.deserialize(serde::de::value::SeqDeserializer::new(
                    self.value.args.iter().map(crate::de::Deserializer::new),
                )),
                Fields::Options => seed.deserialize(serde::de::value::MapDeserializer::new(
                    self.value
                        .options
                        .iter()
                        .map(|(k, v)| (k.as_str(), crate::de::Deserializer::new(v))),
                )),
            },
        }
    }
}
