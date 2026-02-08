use crate::de::GoogleTypeLatLngMapAccess;
use crate::de::ProstTypesTimestampMapAccess;
use crate::google::firestore::v1::{Value, value::ValueType};
use crate::{
    Error, FieldReference, Function, LatLng, Pipeline, Reference, Timestamp, error::ErrorCode,
    value_ext::ValueExt,
};

use super::{
    firestore_array_value_deserializer::FirestoreArrayValueDeserializer,
    firestore_enum_deserializer::FirestoreEnumDeserializer,
    firestore_field_reference_value_deserializer::FirestoreFieldReferenceValueDeserializer,
    firestore_function_value_deserializer::FirestoreFunctionValueDeserializer,
    firestore_map_value_deserializer::FirestoreMapValueDeserializer,
    firestore_pipeline_value_deserializer::FirestorePipelineValueDeserializer,
    firestore_reference_value_deserializer::FirestoreReferenceValueDeserializer,
    firestore_struct_map_value_deserializer::FirestoreStructMapValueDeserializer,
};

/// A Deserializer type which implements [`serde::Deserializer`] for [`Value`].
#[derive(Debug)]
pub struct Deserializer<'a> {
    value: &'a Value,
}

impl<'de> Deserializer<'de> {
    /// Creates a new [`Deserializer`].
    pub fn new(value: &'de Value) -> Self {
        Self { value }
    }
}

impl<'a> serde::Deserializer<'a> for Deserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.value.value_type {
            Some(ref value_type) => match value_type {
                ValueType::NullValue(_) => visitor.visit_unit(),
                ValueType::BooleanValue(v) => visitor.visit_bool(*v),
                ValueType::IntegerValue(v) => visitor.visit_i64(*v),
                ValueType::DoubleValue(v) => visitor.visit_f64(*v),
                ValueType::TimestampValue(_) => visitor.visit_map(
                    ProstTypesTimestampMapAccess::new(self.value.as_timestamp()?),
                ),
                ValueType::StringValue(v) => visitor.visit_str(v),
                ValueType::BytesValue(v) => visitor.visit_bytes(v),
                ValueType::ReferenceValue(v) => visitor.visit_str(v),
                ValueType::GeoPointValue(_) => {
                    visitor.visit_map(GoogleTypeLatLngMapAccess::new(self.value.as_lat_lng()?))
                }
                ValueType::ArrayValue(_) => {
                    visitor.visit_seq(FirestoreArrayValueDeserializer::new(self.value)?)
                }
                ValueType::MapValue(_) => {
                    visitor.visit_map(FirestoreMapValueDeserializer::new(self.value)?)
                }
                ValueType::FieldReferenceValue(v) => visitor.visit_str(v),
                ValueType::FunctionValue(_) => {
                    visitor.visit_map(FirestoreFunctionValueDeserializer::new(self.value)?)
                }
                ValueType::PipelineValue(_) => {
                    visitor.visit_map(FirestorePipelineValueDeserializer::new(self.value)?)
                }
            },
            None => Err(Error::from(ErrorCode::ValueTypeMustBeSome)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_boolean()?;
        visitor.visit_bool(value)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_integer()?;
        visitor.visit_i8(i8::try_from(value).map_err(|_| Error::from(ErrorCode::I8OutOfRange))?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_integer()?;
        visitor.visit_i16(i16::try_from(value).map_err(|_| Error::from(ErrorCode::I16OutOfRange))?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_integer()?;
        visitor.visit_i32(i32::try_from(value).map_err(|_| Error::from(ErrorCode::I32OutOfRange))?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_integer()?;
        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_integer()?;
        visitor.visit_u8(u8::try_from(value).map_err(|_| Error::from(ErrorCode::U8OutOfRange))?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_integer()?;
        visitor.visit_u16(u16::try_from(value).map_err(|_| Error::from(ErrorCode::U16OutOfRange))?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_integer()?;
        visitor.visit_u32(u32::try_from(value).map_err(|_| Error::from(ErrorCode::U32OutOfRange))?)
    }

    fn deserialize_u64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(Error::from(ErrorCode::U64IsNotSupported))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_double()?;
        visitor.visit_f32(value as f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_double()?;
        visitor.visit_f64(value)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_string()?;
        let mut chars = value.chars();
        match (chars.next(), chars.next()) {
            (None, None) => Err(Error::from(ErrorCode::StringIsEmpty)),
            (None, Some(_)) => unreachable!(),
            (Some(c), None) => visitor.visit_char(c),
            (Some(_), Some(_)) => Err(Error::from(ErrorCode::TooManyChars)),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_string()?;
        visitor.visit_str(value)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_bytes()?;
        visitor.visit_bytes(value)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let value = self.value.as_bytes()?;
        visitor.visit_byte_buf(value.to_vec())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.value.value_type()? {
            ValueType::NullValue(_) => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        self.value.as_null()?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        if name == FieldReference::NAME {
            visitor.visit_newtype_struct(FirestoreFieldReferenceValueDeserializer::new(self.value))
        } else if name == Reference::NAME {
            visitor.visit_newtype_struct(FirestoreReferenceValueDeserializer::new(self.value))
        } else {
            visitor.visit_newtype_struct(self)
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_seq(FirestoreArrayValueDeserializer::new(self.value)?)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_seq(FirestoreArrayValueDeserializer::new(self.value)?)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_seq(FirestoreArrayValueDeserializer::new(self.value)?)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_map(FirestoreMapValueDeserializer::new(self.value)?)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        if name == Function::NAME {
            visitor.visit_map(FirestoreFunctionValueDeserializer::new(self.value)?)
        } else if name == LatLng::NAME {
            visitor.visit_map(GoogleTypeLatLngMapAccess::new(self.value.as_lat_lng()?))
        } else if name == Pipeline::NAME {
            visitor.visit_map(FirestorePipelineValueDeserializer::new(self.value)?)
        } else if name == Timestamp::NAME {
            visitor.visit_map(ProstTypesTimestampMapAccess::new(
                self.value.as_timestamp()?,
            ))
        } else {
            visitor.visit_map(FirestoreStructMapValueDeserializer::new(
                self.value, fields,
            )?)
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_enum(FirestoreEnumDeserializer::new(self.value, variants)?)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.value.value_type()? {
            ValueType::StringValue(s) => visitor.visit_str(s.as_str()),
            ValueType::MapValue(_) => {
                let (variant, _) = self.value.as_variant_value()?;
                visitor.visit_str(variant.as_str())
            }
            _ => todo!(),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_unit()
    }
}
