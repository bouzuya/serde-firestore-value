use google::firestore::v1::{value::ValueType, Value};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct Error {
    #[from]
    code: ErrorCode,
}

#[derive(Debug, thiserror::Error)]
enum ErrorCode {
    #[error("i16 out of range")]
    I16OutOfRange,
    #[error("i32 out of range")]
    I32OutOfRange,
    #[error("i8 out of range")]
    I8OutOfRange,
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

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(_msg: T) -> Self {
        todo!()
    }
}

struct FirestoreValueDeserializer<'a> {
    input: &'a Value,
}

impl<'a> serde::Deserializer<'a> for FirestoreValueDeserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.input.value_type.as_ref() {
            None => Err(Error::from(ErrorCode::ValueTypeMustBeSome)),
            Some(ValueType::BooleanValue(value)) => visitor.visit_bool(*value),
            Some(_) => todo!(),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.input.value_type.as_ref() {
            None => Err(Error::from(ErrorCode::ValueTypeMustBeSome)),
            Some(ValueType::IntegerValue(value)) => visitor
                .visit_i8(i8::try_from(*value).map_err(|_| Error::from(ErrorCode::I8OutOfRange))?),
            Some(_) => todo!(),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.input.value_type.as_ref() {
            None => Err(Error::from(ErrorCode::ValueTypeMustBeSome)),
            Some(ValueType::IntegerValue(value)) => visitor.visit_i16(
                i16::try_from(*value).map_err(|_| Error::from(ErrorCode::I16OutOfRange))?,
            ),
            Some(_) => todo!(),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.input.value_type.as_ref() {
            None => Err(Error::from(ErrorCode::ValueTypeMustBeSome)),
            Some(ValueType::IntegerValue(value)) => visitor.visit_i32(
                i32::try_from(*value).map_err(|_| Error::from(ErrorCode::I32OutOfRange))?,
            ),
            Some(_) => todo!(),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.input.value_type.as_ref() {
            None => Err(Error::from(ErrorCode::ValueTypeMustBeSome)),
            Some(ValueType::IntegerValue(value)) => visitor.visit_i64(*value),
            Some(_) => todo!(),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.input.value_type.as_ref() {
            None => Err(Error::from(ErrorCode::ValueTypeMustBeSome)),
            Some(ValueType::IntegerValue(value)) => visitor
                .visit_u8(u8::try_from(*value).map_err(|_| Error::from(ErrorCode::U8OutOfRange))?),
            Some(_) => todo!(),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.input.value_type.as_ref() {
            None => Err(Error::from(ErrorCode::ValueTypeMustBeSome)),
            Some(ValueType::IntegerValue(value)) => visitor.visit_u16(
                u16::try_from(*value).map_err(|_| Error::from(ErrorCode::U16OutOfRange))?,
            ),
            Some(_) => todo!(),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.input.value_type.as_ref() {
            None => Err(Error::from(ErrorCode::ValueTypeMustBeSome)),
            Some(ValueType::IntegerValue(value)) => visitor.visit_u32(
                u32::try_from(*value).map_err(|_| Error::from(ErrorCode::U32OutOfRange))?,
            ),
            Some(_) => todo!(),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        Err(Error::from(ErrorCode::U64IsNotSupported))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.input.value_type.as_ref() {
            None => Err(Error::from(ErrorCode::ValueTypeMustBeSome)),
            Some(ValueType::DoubleValue(value)) => visitor.visit_f64(*value),
            Some(_) => todo!(),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.input.value_type.as_ref() {
            None => Err(Error::from(ErrorCode::ValueTypeMustBeSome)),
            Some(ValueType::StringValue(value)) => visitor.visit_string(value.clone()),
            Some(_) => todo!(),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.input.value_type.as_ref() {
            None => Err(Error::from(ErrorCode::ValueTypeMustBeSome)),
            Some(ValueType::NullValue(_)) => visitor.visit_unit(),
            Some(_) => todo!(),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
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
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use google::firestore::v1::{value::ValueType, Value};

    use super::*;

    fn from_value<'a, T>(v: &'a Value) -> Result<T, Error>
    where
        T: serde::Deserialize<'a>,
    {
        let deserializer = FirestoreValueDeserializer { input: v };
        let t = T::deserialize(deserializer)?;
        Ok(t)
    }

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
    fn test_deserialize_string() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, String>(&Value {
                value_type: Some(ValueType::StringValue("abc".to_string())),
            })?,
            "abc".to_string()
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
    fn test_error_value_type_must_be_some() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, bool>(&Value { value_type: None })
                .unwrap_err()
                .to_string(),
            "value type must be some"
        );
        Ok(())
    }
}
