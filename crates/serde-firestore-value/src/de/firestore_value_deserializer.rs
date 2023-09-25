use google::firestore::v1::{value::ValueType, MapValue, Value};

use crate::typ::{my_lat_lng::MyLatLng, my_reference::MyReference, my_timestamp::MyTimestamp};

use super::{
    error::{Error, ErrorCode},
    firestore_array_value_deserializer::FirestoreArrayValueDeserializer,
    firestore_enum_deserializer::FirestoreEnumDeserializer,
    firestore_geo_point_value_deserializer::FirestoreGeoPointValueDeserializer,
    firestore_map_value_deserializer::FirestoreMapValueDeserializer,
    firestore_reference_value_deserializer::FirestoreReferenceValueDeserializer,
    firestore_struct_map_value_deserializer::FirestoreStructMapValueDeserializer,
    firestore_timestamp_value_deserializer::FirestoreTimestampValueDeserializer,
    value_ext::ValueExt,
};

pub(super) struct FirestoreValueDeserializer<'a> {
    value: &'a Value,
}

impl<'de> FirestoreValueDeserializer<'de> {
    pub(super) fn new(value: &'de Value) -> Self {
        Self { value }
    }
}

impl<'a> serde::Deserializer<'a> for FirestoreValueDeserializer<'a> {
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
                ValueType::TimestampValue(_) => {
                    visitor.visit_map(FirestoreTimestampValueDeserializer::new(self.value)?)
                }
                ValueType::StringValue(v) => visitor.visit_str(v),
                ValueType::BytesValue(v) => visitor.visit_bytes(v),
                ValueType::ReferenceValue(v) => visitor.visit_str(v),
                ValueType::GeoPointValue(_) => {
                    visitor.visit_map(FirestoreGeoPointValueDeserializer::new(self.value)?)
                }
                ValueType::ArrayValue(_) => {
                    visitor.visit_seq(FirestoreArrayValueDeserializer::new(self.value)?)
                }
                ValueType::MapValue(_) => {
                    visitor.visit_map(FirestoreMapValueDeserializer::new(self.value)?)
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
        if name == MyReference::NAME {
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
        if name == MyLatLng::NAME {
            visitor.visit_map(FirestoreGeoPointValueDeserializer::new(self.value)?)
        } else if name == MyTimestamp::NAME {
            visitor.visit_map(FirestoreTimestampValueDeserializer::new(self.value)?)
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
            ValueType::MapValue(MapValue { fields }) => {
                if fields.len() != 1 {
                    todo!()
                }
                let (variant_name, _) = fields.iter().next().unwrap();
                visitor.visit_str(variant_name.as_str())
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
