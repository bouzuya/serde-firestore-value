use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};
use serde::de::{value::StringDeserializer, MapAccess, SeqAccess};

pub fn from_value<'a, T>(v: &'a Value) -> Result<T, Error>
where
    T: serde::Deserialize<'a>,
{
    let deserializer = FirestoreValueDeserializer { value: v };
    let t = T::deserialize(deserializer)?;
    Ok(t)
}

trait ValueExt {
    fn as_null(&self) -> Result<(), Error>;
    fn as_boolean(&self) -> Result<bool, Error>;
    fn as_integer(&self) -> Result<i64, Error>;
    fn as_double(&self) -> Result<f64, Error>;
    fn as_string(&self) -> Result<&String, Error>;
    fn as_bytes(&self) -> Result<&[u8], Error>;
    fn as_array(&self) -> Result<&ArrayValue, Error>;
    fn as_map(&self) -> Result<&MapValue, Error>;
    fn as_variant_value(&self, variants: &'static [&'static str]) -> Result<&Value, Error>;
    fn value_type(&self) -> Result<&ValueType, Error>;
}

impl ValueExt for Value {
    fn as_null(&self) -> Result<(), Error> {
        match self.value_type()? {
            ValueType::NullValue(_) => Ok(()),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Null)),
        }
    }

    fn as_boolean(&self) -> Result<bool, Error> {
        match self.value_type()? {
            ValueType::BooleanValue(value) => Ok(*value),
            value_type => Err(Error::invalid_value_type(
                value_type,
                ValueTypeName::Boolean,
            )),
        }
    }

    fn as_integer(&self) -> Result<i64, Error> {
        match self.value_type()? {
            ValueType::IntegerValue(value) => Ok(*value),
            value_type => Err(Error::invalid_value_type(
                value_type,
                ValueTypeName::Integer,
            )),
        }
    }

    fn as_double(&self) -> Result<f64, Error> {
        match self.value_type()? {
            ValueType::DoubleValue(value) => Ok(*value),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Double)),
        }
    }

    fn as_string(&self) -> Result<&String, Error> {
        match self.value_type()? {
            ValueType::StringValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::String)),
        }
    }

    fn as_bytes(&self) -> Result<&[u8], Error> {
        match self.value_type()? {
            ValueType::BytesValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Bytes)),
        }
    }

    fn as_array(&self) -> Result<&ArrayValue, Error> {
        match self.value_type()? {
            ValueType::ArrayValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Array)),
        }
    }

    fn as_map(&self) -> Result<&MapValue, Error> {
        match self.value_type()? {
            ValueType::MapValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Map)),
        }
    }

    fn as_variant_value(&self, variants: &'static [&'static str]) -> Result<&Value, Error> {
        let MapValue { fields } = self.as_map()?;
        if fields.len() != 1 {
            todo!()
        }
        let (variant, value) = fields.iter().next().expect("fields must have an entry");
        if !variants.contains(&variant.as_str()) {
            todo!()
        }
        Ok(value)
    }

    fn value_type(&self) -> Result<&ValueType, Error> {
        self.value_type
            .as_ref()
            .ok_or_else(|| Error::from(ErrorCode::ValueTypeMustBeSome))
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error {
    #[from]
    code: ErrorCode,
}

#[derive(Debug, thiserror::Error)]
enum ErrorCode {
    #[error("{0}")]
    Custom(String),
    #[error("deserialize_any is not supported")]
    DeserializeAnyIsNotSupported,
    #[error("i16 out of range")]
    I16OutOfRange,
    #[error("i32 out of range")]
    I32OutOfRange,
    #[error("i8 out of range")]
    I8OutOfRange,
    #[error("string is empty")]
    StringIsEmpty,
    #[error("too many chars")]
    TooManyChars,
    #[error("u16 out of range")]
    U16OutOfRange,
    #[error("u32 out of range")]
    U32OutOfRange,
    #[error("u64 is not supported")]
    U64IsNotSupported,
    #[error("u8 out of range")]
    U8OutOfRange,
    #[error("value type must be some")]
    ValueTypeMustBeSome,
}

impl Error {
    fn invalid_value_type(value_type: &ValueType, expected: ValueTypeName) -> Self {
        <Self as serde::de::Error>::invalid_type(
            serde::de::Unexpected::Other(value_type.name().as_str()),
            &expected.as_str(),
        )
    }
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::from(ErrorCode::Custom(msg.to_string()))
    }
}

trait ValueTypeExt {
    fn name(&self) -> ValueTypeName;
}

impl ValueTypeExt for ValueType {
    fn name(&self) -> ValueTypeName {
        match self {
            ValueType::NullValue(_) => ValueTypeName::Null,
            ValueType::BooleanValue(_) => ValueTypeName::Boolean,
            ValueType::IntegerValue(_) => ValueTypeName::Integer,
            ValueType::DoubleValue(_) => ValueTypeName::Double,
            ValueType::TimestampValue(_) => ValueTypeName::Timestamp,
            ValueType::StringValue(_) => ValueTypeName::String,
            ValueType::BytesValue(_) => ValueTypeName::Bytes,
            ValueType::ReferenceValue(_) => ValueTypeName::Reference,
            ValueType::GeoPointValue(_) => ValueTypeName::GeoPoint,
            ValueType::ArrayValue(_) => ValueTypeName::Array,
            ValueType::MapValue(_) => ValueTypeName::Map,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ValueTypeName {
    Null,
    Boolean,
    Integer,
    Double,
    Timestamp,
    String,
    Bytes,
    Reference,
    GeoPoint,
    Array,
    Map,
}

impl ValueTypeName {
    fn as_str(&self) -> &'static str {
        match self {
            ValueTypeName::Null => "null value",
            ValueTypeName::Boolean => "boolean value",
            ValueTypeName::Integer => "integer value",
            ValueTypeName::Double => "double value",
            ValueTypeName::Timestamp => "timestamp value",
            ValueTypeName::String => "string value",
            ValueTypeName::Bytes => "bytes value",
            ValueTypeName::Reference => "reference value",
            ValueTypeName::GeoPoint => "geo point value",
            ValueTypeName::Array => "array value",
            ValueTypeName::Map => "map value",
        }
    }
}

struct FirestoreValueDeserializer<'a> {
    value: &'a Value,
}

impl<'a> serde::Deserializer<'a> for FirestoreValueDeserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(Error::from(ErrorCode::DeserializeAnyIsNotSupported))
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
        let value = self.value.as_string()?;
        visitor.visit_string(value.clone())
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
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_seq(FirestoreArrayValueDeserializer {
            index: 0,
            value: self.value,
        })
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        visitor.visit_seq(FirestoreArrayValueDeserializer {
            index: 0,
            value: self.value,
        })
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
        visitor.visit_seq(FirestoreArrayValueDeserializer {
            index: 0,
            value: self.value,
        })
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let MapValue { fields } = self.value.as_map()?;
        visitor.visit_map(FirestoreMapValueDeserializer {
            iter: fields.iter(),
            next_value: None,
        })
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        let MapValue { fields: values } = self.value.as_map()?;
        visitor.visit_map(FirestoreStructMapValueDeserializer {
            fields,
            index: 0,
            next_value: None,
            values,
        })
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
        visitor.visit_enum(FirestoreEnumDeserializer {
            value: self.value,
            variants,
        })
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

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(Error::from(ErrorCode::DeserializeAnyIsNotSupported))
    }
}

struct FirestoreArrayValueDeserializer<'de> {
    index: usize,
    value: &'de Value,
}

impl<'de> SeqAccess<'de> for FirestoreArrayValueDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let ArrayValue { values } = self.value.as_array()?;
        if self.index < values.len() {
            let value = &values[self.index];
            self.index += 1;
            seed.deserialize(FirestoreValueDeserializer { value })
                .map(Some)
        } else {
            Ok(None)
        }
    }
}

struct FirestoreMapValueDeserializer<'de> {
    iter: std::collections::hash_map::Iter<'de, String, Value>,
    next_value: Option<&'de Value>,
}

impl<'de> MapAccess<'de> for FirestoreMapValueDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                if self.next_value.is_none() {
                    self.next_value = Some(value);
                    seed.deserialize(StringDeserializer::new(key.clone()))
                        .map(Some)
                } else {
                    unreachable!()
                }
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if let Some(value) = self.next_value.take() {
            seed.deserialize(FirestoreValueDeserializer { value })
        } else {
            unreachable!()
        }
    }
}

struct FirestoreStructMapValueDeserializer<'de> {
    fields: &'static [&'static str],
    index: usize,
    next_value: Option<&'de Value>,
    values: &'de HashMap<String, Value>,
}

impl<'de> MapAccess<'de> for FirestoreStructMapValueDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index >= self.fields.len() {
            Ok(None)
        } else {
            let field = self.fields[self.index];
            self.index += 1;
            if self.next_value.is_none() {
                self.next_value = self.values.get(field);
                seed.deserialize(StringDeserializer::new(field.to_string()))
                    .map(Some)
            } else {
                unreachable!()
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if let Some(value) = self.next_value.take() {
            seed.deserialize(FirestoreValueDeserializer { value })
        } else {
            unreachable!()
        }
    }
}

struct FirestoreEnumDeserializer<'de> {
    value: &'de Value,
    variants: &'static [&'static str],
}

impl<'de> serde::de::EnumAccess<'de> for FirestoreEnumDeserializer<'de> {
    type Error = Error;
    type Variant = FirestoreEnumDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(FirestoreValueDeserializer { value: self.value })
            .map(|v| (v, self))
    }
}

impl<'de> serde::de::VariantAccess<'de> for FirestoreEnumDeserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value.value_type()? {
            ValueType::StringValue(variant_name) => {
                if self.variants.contains(&variant_name.as_str()) {
                    Ok(())
                } else {
                    todo!()
                }
            }
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::String)),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let value = self.value.as_variant_value(self.variants)?;
        seed.deserialize(FirestoreValueDeserializer { value })
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value = self.value.as_variant_value(self.variants)?;
        value.as_array()?;
        visitor.visit_seq(FirestoreArrayValueDeserializer { index: 0, value })
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value = self.value.as_variant_value(self.variants)?;
        let MapValue { fields } = value.as_map()?;
        visitor.visit_map(FirestoreMapValueDeserializer {
            iter: fields.iter(),
            next_value: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use google::firestore::v1::{value::ValueType, Value};

    use super::*;

    #[test]
    fn test_deserialize_bool() -> anyhow::Result<()> {
        assert!(from_value::<'_, bool>(&Value {
            value_type: Some(ValueType::BooleanValue(true)),
        })?);
        assert!(!from_value::<'_, bool>(&Value {
            value_type: Some(ValueType::BooleanValue(false)),
        })?);
        Ok(())
    }

    #[test]
    fn test_deserialize_i8() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, i8>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::from(i8::MAX))),
            })?,
            i8::MAX
        );
        assert_eq!(
            from_value::<'_, i8>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::from(i8::MIN))),
            })?,
            i8::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_i16() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, i16>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::from(i16::MAX))),
            })?,
            i16::MAX
        );
        assert_eq!(
            from_value::<'_, i16>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::from(i16::MIN))),
            })?,
            i16::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_i32() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, i32>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::from(i32::MAX))),
            })?,
            i32::MAX
        );
        assert_eq!(
            from_value::<'_, i32>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::from(i32::MIN))),
            })?,
            i32::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_i64() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, i64>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::MAX)),
            })?,
            i64::MAX
        );
        assert_eq!(
            from_value::<'_, i64>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::MIN)),
            })?,
            i64::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_u8() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, u8>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u8::MAX))),
            })?,
            u8::MAX
        );
        assert_eq!(
            from_value::<'_, u8>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u8::MIN))),
            })?,
            u8::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_u16() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, u16>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u16::MAX))),
            })?,
            u16::MAX
        );
        assert_eq!(
            from_value::<'_, u16>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u16::MIN))),
            })?,
            u16::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_u32() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, u32>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u32::MAX))),
            })?,
            u32::MAX
        );
        assert_eq!(
            from_value::<'_, u32>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u32::MIN))),
            })?,
            u32::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_u64() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, u64>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::try_from(u64::MIN)?)),
            })
            .unwrap_err()
            .to_string(),
            "u64 is not supported"
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_f32() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, f32>(&Value {
                value_type: Some(ValueType::DoubleValue(f64::from(f32::MAX))),
            })?,
            f32::MAX
        );
        assert_eq!(
            from_value::<'_, f32>(&Value {
                value_type: Some(ValueType::DoubleValue(f64::from(f32::MIN))),
            })?,
            f32::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_f64() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, f64>(&Value {
                value_type: Some(ValueType::DoubleValue(f64::MAX)),
            })?,
            f64::MAX
        );
        assert_eq!(
            from_value::<'_, f64>(&Value {
                value_type: Some(ValueType::DoubleValue(f64::MIN)),
            })?,
            f64::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_char() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, char>(&Value {
                value_type: Some(ValueType::StringValue("a".to_string())),
            })?,
            'a'
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_str() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, String>(&Value {
                value_type: Some(ValueType::StringValue("abc".to_string())),
            })?,
            // "abc".to_string()
            "abc"
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_string() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, String>(&Value {
                value_type: Some(ValueType::StringValue("abc".to_string())),
            })?,
            // "abc"
            "abc".to_string()
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_option() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, Option<bool>>(&Value {
                value_type: Some(ValueType::BooleanValue(true)),
            })?,
            Some(true)
        );
        assert_eq!(
            from_value::<'_, Option<bool>>(&Value {
                value_type: Some(ValueType::NullValue(0_i32)),
            })?,
            None,
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_unit() -> anyhow::Result<()> {
        from_value::<'_, ()>(&Value {
            value_type: Some(ValueType::NullValue(0_i32)),
        })?;
        Ok(())
    }

    #[test]
    fn test_deserialize_unit_struct() -> anyhow::Result<()> {
        #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
        struct Unit;
        assert_eq!(
            from_value::<'_, Unit>(&Value {
                value_type: Some(ValueType::NullValue(0_i32)),
            })?,
            Unit
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_newtype_struct() -> anyhow::Result<()> {
        #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
        struct Millimeters(u8);
        assert_eq!(
            from_value::<'_, Millimeters>(&Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u8::MAX))),
            })?,
            Millimeters(u8::MAX)
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_seq() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, Vec<i64>>(&Value {
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: vec![
                        Value {
                            value_type: Some(ValueType::IntegerValue(1_i64))
                        },
                        Value {
                            value_type: Some(ValueType::IntegerValue(2_i64))
                        },
                        Value {
                            value_type: Some(ValueType::IntegerValue(3_i64))
                        },
                    ]
                }))
            })?,
            vec![1_i64, 2_i64, 3_i64]
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_tuple() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, (bool, i64)>(&Value {
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: vec![
                        Value {
                            value_type: Some(ValueType::BooleanValue(true))
                        },
                        Value {
                            value_type: Some(ValueType::IntegerValue(1_i64))
                        },
                    ]
                }))
            })?,
            (true, 1_i64)
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_tuple_struct() -> anyhow::Result<()> {
        #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
        struct Rgb(u8, u8, u8);
        assert_eq!(
            from_value::<'_, Rgb>(&Value {
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: vec![
                        Value {
                            value_type: Some(ValueType::IntegerValue(1_i64))
                        },
                        Value {
                            value_type: Some(ValueType::IntegerValue(2_i64))
                        },
                        Value {
                            value_type: Some(ValueType::IntegerValue(3_i64))
                        },
                    ]
                }))
            })?,
            Rgb(1_u8, 2_u8, 3_u8)
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_map() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, BTreeMap<String, i64>>(&Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: {
                        let mut map = HashMap::new();
                        map.insert(
                            "k1".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(1_i64)),
                            },
                        );
                        map.insert(
                            "k2".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(2_i64)),
                            },
                        );
                        map
                    }
                }))
            })?,
            {
                let mut map = BTreeMap::new();
                map.insert("k1".to_string(), 1_i64);
                map.insert("k2".to_string(), 2_i64);
                map
            }
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_struct() -> anyhow::Result<()> {
        #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
        struct S {
            r: u8,
            g: u8,
            b: u8,
        }
        assert_eq!(
            from_value::<'_, S>(&Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: {
                        let mut map = HashMap::new();
                        map.insert(
                            "r".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(1_i64)),
                            },
                        );
                        map.insert(
                            "g".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(2_i64)),
                            },
                        );
                        map.insert(
                            "b".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(3_i64)),
                            },
                        );
                        map
                    }
                }))
            })?,
            S {
                r: 1_u8,
                g: 2_u8,
                b: 3_u8
            }
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_enum() -> anyhow::Result<()> {
        {
            // unit_variant
            #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
            enum E {
                A,
                B,
            }
            assert_eq!(
                from_value::<'_, E>(&Value {
                    value_type: Some(ValueType::StringValue("A".to_string()))
                })?,
                E::A
            );
            assert_eq!(
                from_value::<'_, E>(&Value {
                    value_type: Some(ValueType::StringValue("B".to_string()))
                })?,
                E::B
            );
        }

        {
            // newtype_variant
            #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
            enum E {
                A(u8),
                B(u8),
            }
            assert_eq!(
                from_value::<'_, E>(&Value {
                    value_type: Some(ValueType::MapValue(MapValue {
                        fields: {
                            let mut map = HashMap::new();
                            map.insert(
                                "A".to_string(),
                                Value {
                                    value_type: Some(ValueType::IntegerValue(1_i64)),
                                },
                            );
                            map
                        }
                    }))
                })?,
                E::A(1_u8)
            );
            assert_eq!(
                from_value::<'_, E>(&Value {
                    value_type: Some(ValueType::MapValue(MapValue {
                        fields: {
                            let mut map = HashMap::new();
                            map.insert(
                                "B".to_string(),
                                Value {
                                    value_type: Some(ValueType::IntegerValue(2_i64)),
                                },
                            );
                            map
                        }
                    }))
                })?,
                E::B(2_u8)
            );
        }

        {
            // tuple_variant
            #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
            enum E {
                T(u8, u8),
                U(u8, u8),
            }
            assert_eq!(
                from_value::<'_, E>(&Value {
                    value_type: Some(ValueType::MapValue(MapValue {
                        fields: {
                            let mut map = HashMap::new();
                            map.insert(
                                "T".to_string(),
                                Value {
                                    value_type: Some(ValueType::ArrayValue(ArrayValue {
                                        values: vec![
                                            Value {
                                                value_type: Some(ValueType::IntegerValue(1_i64)),
                                            },
                                            Value {
                                                value_type: Some(ValueType::IntegerValue(2_i64)),
                                            },
                                        ],
                                    })),
                                },
                            );
                            map
                        }
                    }))
                })?,
                E::T(1_u8, 2_u8)
            );
        }

        {
            // struct_variant
            #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
            enum E {
                S { r: u8, g: u8, b: u8 },
            }
            assert_eq!(
                from_value::<'_, E>(&Value {
                    value_type: Some(ValueType::MapValue(MapValue {
                        fields: {
                            let mut map = HashMap::new();
                            map.insert(
                                "S".to_string(),
                                Value {
                                    value_type: Some(ValueType::MapValue(MapValue {
                                        fields: {
                                            let mut map = HashMap::new();
                                            map.insert(
                                                "r".to_string(),
                                                Value {
                                                    value_type: Some(ValueType::IntegerValue(
                                                        1_i64,
                                                    )),
                                                },
                                            );
                                            map.insert(
                                                "g".to_string(),
                                                Value {
                                                    value_type: Some(ValueType::IntegerValue(
                                                        2_i64,
                                                    )),
                                                },
                                            );
                                            map.insert(
                                                "b".to_string(),
                                                Value {
                                                    value_type: Some(ValueType::IntegerValue(
                                                        3_i64,
                                                    )),
                                                },
                                            );
                                            map
                                        },
                                    })),
                                },
                            );
                            map
                        }
                    }))
                })?,
                E::S { r: 1, g: 2, b: 3 },
            );
        }

        Ok(())
    }

    #[test]
    fn test_error_value_type_must_be_some() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, bool>(&Value { value_type: None })
                .unwrap_err()
                .to_string(),
            "value type must be some"
        );
        Ok(())
    }

    #[test]
    fn test_error_invalid_type() -> anyhow::Result<()> {
        // expected boolean value
        assert_eq!(
            from_value::<'_, bool>(&Value {
                value_type: Some(ValueType::IntegerValue(1_i64)),
            })
            .unwrap_err()
            .to_string(),
            "invalid type: integer value, expected boolean value"
        );

        // expected integer value
        assert_eq!(
            from_value::<'_, i8>(&Value {
                value_type: Some(ValueType::BooleanValue(true)),
            })
            .unwrap_err()
            .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        assert_eq!(
            from_value::<'_, i16>(&Value {
                value_type: Some(ValueType::BooleanValue(true)),
            })
            .unwrap_err()
            .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        assert_eq!(
            from_value::<'_, i32>(&Value {
                value_type: Some(ValueType::BooleanValue(true)),
            })
            .unwrap_err()
            .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        assert_eq!(
            from_value::<'_, i64>(&Value {
                value_type: Some(ValueType::BooleanValue(true)),
            })
            .unwrap_err()
            .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        assert_eq!(
            from_value::<'_, u8>(&Value {
                value_type: Some(ValueType::BooleanValue(true)),
            })
            .unwrap_err()
            .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        assert_eq!(
            from_value::<'_, u16>(&Value {
                value_type: Some(ValueType::BooleanValue(true)),
            })
            .unwrap_err()
            .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        assert_eq!(
            from_value::<'_, u32>(&Value {
                value_type: Some(ValueType::BooleanValue(true)),
            })
            .unwrap_err()
            .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        // u64 is not supported

        // expected double value
        assert_eq!(
            from_value::<'_, f32>(&Value {
                value_type: Some(ValueType::BooleanValue(true)),
            })
            .unwrap_err()
            .to_string(),
            "invalid type: boolean value, expected double value"
        );
        assert_eq!(
            from_value::<'_, f64>(&Value {
                value_type: Some(ValueType::BooleanValue(true)),
            })
            .unwrap_err()
            .to_string(),
            "invalid type: boolean value, expected double value"
        );

        // TODO: ...
        Ok(())
    }
}
