use google::firestore::v1::{value::ValueType, Value};

#[derive(Debug, thiserror::Error)]
#[error("error")]
struct Error;

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
            None => todo!(),
            Some(ValueType::BooleanValue(value)) => visitor.visit_bool(*value),
            Some(_) => todo!(),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        match self.input.value_type.as_ref() {
            None => todo!(),
            Some(ValueType::IntegerValue(value)) => visitor.visit_i64(*value),
            Some(_) => todo!(),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'a>,
    {
        todo!()
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
            None => todo!(),
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
            None => todo!(),
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
            None => todo!(),
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
}
