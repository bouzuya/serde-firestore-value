pub(crate) struct GoogleTypeLatLngMapAccess<'de> {
    index: usize,
    lat_lng: &'de crate::google::r#type::LatLng,
}

impl<'de> GoogleTypeLatLngMapAccess<'de> {
    pub(crate) fn new(lat_lng: &'de crate::google::r#type::LatLng) -> Self {
        Self { index: 0, lat_lng }
    }
}

impl<'de> serde::de::MapAccess<'de> for GoogleTypeLatLngMapAccess<'de> {
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
                .deserialize(serde::de::value::StrDeserializer::new(crate::LatLng::NAME))
                .map(Some),
            2 => seed
                .deserialize(serde::de::value::StrDeserializer::new("latitude"))
                .map(Some),
            3 => seed
                .deserialize(serde::de::value::StrDeserializer::new("longitude"))
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
            2 => seed.deserialize(serde::de::value::F64Deserializer::new(
                self.lat_lng.latitude,
            )),
            3 => seed.deserialize(serde::de::value::F64Deserializer::new(
                self.lat_lng.longitude,
            )),
            _ => unreachable!(),
        }
    }
}
