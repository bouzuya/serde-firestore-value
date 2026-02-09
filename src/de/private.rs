use crate::Error;

pub(super) struct NewtypeStructMapAccess<'a> {
    name: &'static str,
    value: Option<&'a str>,
    index: usize,
}

impl<'a> NewtypeStructMapAccess<'a> {
    pub(super) fn new(name: &'static str, value: &'a str) -> Self {
        Self {
            name,
            value: Some(value),
            index: 0,
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for NewtypeStructMapAccess<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index >= 1 {
            return Ok(None);
        }
        self.index += 1;
        seed.deserialize(serde::de::value::StrDeserializer::new(self.name))
            .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(v) => seed.deserialize(serde::de::value::StrDeserializer::new(v)),
            None => unreachable!(),
        }
    }
}
