use crate::Error;
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
            Some(ValueType::ArrayValue(arr)) => visitor.visit_seq(ValueSeqAccess {
                iter: arr.values.iter(),
            }),
            Some(ValueType::MapValue(map)) => visitor.visit_map(ValueMapDeserializerAccess {
                iter: map.fields.iter(),
                value: None,
            }),
            Some(ValueType::TimestampValue(ts)) => {
                visitor.visit_map(TimestampMapAccess::new(ts.seconds, ts.nanos))
            }
            Some(ValueType::ReferenceValue(v)) => {
                visitor.visit_map(NewtypeStructMapAccess::new(crate::Reference::NAME, v))
            }
            Some(ValueType::GeoPointValue(geo)) => {
                visitor.visit_map(GeoPointMapAccess::new(geo.latitude, geo.longitude))
            }
            Some(ValueType::FieldReferenceValue(v)) => {
                visitor.visit_map(NewtypeStructMapAccess::new(crate::FieldReference::NAME, v))
            }
            Some(ValueType::FunctionValue(f)) => visitor.visit_map(FunctionValueMapAccess::new(f)),
            Some(ValueType::PipelineValue(p)) => visitor.visit_map(PipelineValueMapAccess::new(p)),
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

pub(super) struct ValueSeqAccess<'a> {
    pub iter: std::slice::Iter<'a, Value>,
}

impl<'de> serde::de::SeqAccess<'de> for ValueSeqAccess<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(ValueDeserializer(value)).map(Some),
            None => Ok(None),
        }
    }
}

#[cfg(feature = "btree-map")]
pub(super) struct ValueMapDeserializerAccess<'a> {
    pub iter: std::collections::btree_map::Iter<'a, String, Value>,
    pub value: Option<&'a Value>,
}

#[cfg(feature = "hash-map")]
pub(super) struct ValueMapDeserializerAccess<'a> {
    pub iter: std::collections::hash_map::Iter<'a, String, Value>,
    pub value: Option<&'a Value>,
}

impl<'de> serde::de::MapAccess<'de> for ValueMapDeserializerAccess<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(serde::de::value::StrDeserializer::new(key.as_str()))
                    .map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(ValueDeserializer(value)),
            None => unreachable!(),
        }
    }
}

pub(super) struct TimestampMapAccess {
    index: usize,
    seconds: i64,
    nanos: i32,
}

impl TimestampMapAccess {
    pub(super) fn new(seconds: i64, nanos: i32) -> Self {
        Self {
            index: 0,
            seconds,
            nanos,
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for TimestampMapAccess {
    type Error = Error;

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
                .deserialize(serde::de::value::StrDeserializer::new(
                    crate::Timestamp::NAME,
                ))
                .map(Some),
            2 => seed
                .deserialize(serde::de::value::StrDeserializer::new("seconds"))
                .map(Some),
            3 => seed
                .deserialize(serde::de::value::StrDeserializer::new("nanos"))
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
            2 => seed.deserialize(serde::de::value::I64Deserializer::new(self.seconds)),
            3 => seed.deserialize(serde::de::value::I32Deserializer::new(self.nanos)),
            _ => unreachable!(),
        }
    }
}

pub(super) struct GeoPointMapAccess {
    index: usize,
    latitude: f64,
    longitude: f64,
}

impl GeoPointMapAccess {
    pub(super) fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            index: 0,
            latitude,
            longitude,
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for GeoPointMapAccess {
    type Error = Error;

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
            2 => seed.deserialize(serde::de::value::F64Deserializer::new(self.latitude)),
            3 => seed.deserialize(serde::de::value::F64Deserializer::new(self.longitude)),
            _ => unreachable!(),
        }
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

pub(super) struct FunctionValueMapAccess<'a> {
    function: &'a crate::google::firestore::v1::Function,
    index: usize,
}

impl<'a> FunctionValueMapAccess<'a> {
    pub(super) fn new(function: &'a crate::google::firestore::v1::Function) -> Self {
        Self { function, index: 0 }
    }
}

impl<'de> serde::de::MapAccess<'de> for FunctionValueMapAccess<'de> {
    type Error = Error;

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
                .deserialize(serde::de::value::StrDeserializer::new(
                    crate::Function::NAME,
                ))
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
            2 => seed.deserialize(serde::de::value::StrDeserializer::new(&self.function.name)),
            3 => seed.deserialize(serde::de::value::SeqDeserializer::new(
                self.function.args.iter().map(ValueDeserializer),
            )),
            4 => seed.deserialize(serde::de::value::MapDeserializer::new(
                self.function
                    .options
                    .iter()
                    .map(|(k, v)| (k.as_str(), ValueDeserializer(v))),
            )),
            _ => unreachable!(),
        }
    }
}

pub(super) struct PipelineValueMapAccess<'a> {
    index: usize,
    pipeline: &'a crate::google::firestore::v1::Pipeline,
}

impl<'a> PipelineValueMapAccess<'a> {
    pub(super) fn new(pipeline: &'a crate::google::firestore::v1::Pipeline) -> Self {
        Self { index: 0, pipeline }
    }
}

impl<'de> serde::de::MapAccess<'de> for PipelineValueMapAccess<'de> {
    type Error = Error;

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
                self.pipeline.stages.iter().map(StageDeserializer),
            )),
            _ => unreachable!(),
        }
    }
}

/// A deserializer for a single Stage
pub(super) struct StageDeserializer<'a>(pub &'a crate::google::firestore::v1::pipeline::Stage);

impl<'de> serde::de::IntoDeserializer<'de, Error> for StageDeserializer<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> serde::Deserializer<'de> for StageDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(StageMapAccess::new(self.0))
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

/// MapAccess for Stage (similar to FunctionValueMapAccess)
pub(super) struct StageMapAccess<'a> {
    index: usize,
    stage: &'a crate::google::firestore::v1::pipeline::Stage,
}

impl<'a> StageMapAccess<'a> {
    pub(super) fn new(stage: &'a crate::google::firestore::v1::pipeline::Stage) -> Self {
        Self { index: 0, stage }
    }
}

impl<'de> serde::de::MapAccess<'de> for StageMapAccess<'de> {
    type Error = Error;

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
                self.stage.args.iter().map(ValueDeserializer),
            )),
            4 => seed.deserialize(serde::de::value::MapDeserializer::new(
                self.stage
                    .options
                    .iter()
                    .map(|(k, v)| (k.as_str(), ValueDeserializer(v))),
            )),
            _ => unreachable!(),
        }
    }
}
