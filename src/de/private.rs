use crate::Error;
use crate::de::GoogleFirestoreFunctionMapAccess;
use crate::de::GoogleFirestorePipelineMapAccess;
use crate::de::GoogleTypeLatLngMapAccess;
use crate::de::ProstTypesTimestampMapAccess;
use crate::google::firestore::v1::Value;

/// A simple deserializer that wraps a Value and returns it directly
pub(super) struct ValueDeserializer<'a>(pub &'a Value);

impl<'de> serde::de::IntoDeserializer<'de, Error> for ValueDeserializer<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> serde::Deserializer<'de> for ValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        use crate::google::firestore::v1::value::ValueType;

        match &self.0.value_type {
            None => visitor.visit_none(),
            Some(ValueType::NullValue(_)) => visitor.visit_none(),
            Some(ValueType::BooleanValue(v)) => visitor.visit_bool(*v),
            Some(ValueType::IntegerValue(v)) => visitor.visit_i64(*v),
            Some(ValueType::DoubleValue(v)) => visitor.visit_f64(*v),
            Some(ValueType::StringValue(v)) => visitor.visit_str(v),
            Some(ValueType::BytesValue(v)) => visitor.visit_bytes(v),
            Some(ValueType::ArrayValue(v)) => visitor.visit_seq(
                serde::de::value::SeqDeserializer::new(v.values.iter().map(ValueDeserializer)),
            ),
            Some(ValueType::MapValue(map)) => {
                visitor.visit_map(serde::de::value::MapDeserializer::new(
                    map.fields
                        .iter()
                        .map(|(k, v)| (k.as_str(), ValueDeserializer(v))),
                ))
            }
            Some(ValueType::TimestampValue(v)) => {
                visitor.visit_map(ProstTypesTimestampMapAccess::new(v))
            }
            Some(ValueType::ReferenceValue(v)) => {
                visitor.visit_map(NewtypeStructMapAccess::new(crate::Reference::NAME, v))
            }
            Some(ValueType::GeoPointValue(lat_lng)) => {
                visitor.visit_map(GoogleTypeLatLngMapAccess::new(lat_lng))
            }
            Some(ValueType::FieldReferenceValue(v)) => {
                visitor.visit_map(NewtypeStructMapAccess::new(crate::FieldReference::NAME, v))
            }
            Some(ValueType::FunctionValue(v)) => {
                visitor.visit_map(GoogleFirestoreFunctionMapAccess::new(v))
            }
            Some(ValueType::PipelineValue(v)) => {
                visitor.visit_map(GoogleFirestorePipelineMapAccess::new(v))
            }
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

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
