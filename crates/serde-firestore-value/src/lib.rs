#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use google::firestore::v1::{value::ValueType, Value};
    use serde::{
        ser::{
            SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
            SerializeTupleStruct, SerializeTupleVariant,
        },
        Serialize, Serializer,
    };

    #[derive(Debug, thiserror::Error)]
    #[error("error")]
    struct Error;

    impl serde::ser::Error for Error {
        fn custom<T: Display>(_msg: T) -> Self {
            todo!()
        }
    }

    struct FirestoreValueSerializer {
        output: Value,
    }

    impl<'a> Serializer for &'a mut FirestoreValueSerializer {
        type Ok = ();

        type Error = Error;

        type SerializeSeq = Self;

        type SerializeTuple = Self;

        type SerializeTupleStruct = Self;

        type SerializeTupleVariant = Self;

        type SerializeMap = Self;

        type SerializeStruct = Self;

        type SerializeStructVariant = Self;

        fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
            self.output.value_type = Some(ValueType::BooleanValue(v));
            Ok(())
        }

        fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
            self.serialize_i64(i64::from(v))
        }

        fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
            self.serialize_i64(i64::from(v))
        }

        fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
            self.serialize_i64(i64::from(v))
        }

        fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
            self.output.value_type = Some(ValueType::IntegerValue(v));
            Ok(())
        }

        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
            self.serialize_f64(f64::from(v))
        }

        fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
            self.output.value_type = Some(ValueType::DoubleValue(v));
            Ok(())
        }

        fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
            // TODO: length check
            self.output.value_type = Some(ValueType::StringValue(v.to_string()));
            Ok(())
        }

        fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_newtype_struct<T: ?Sized>(
            self,
            name: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn serialize_newtype_variant<T: ?Sized>(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            todo!()
        }

        fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
            todo!()
        }

        fn serialize_tuple_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            todo!()
        }

        fn serialize_tuple_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            todo!()
        }

        fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            todo!()
        }

        fn serialize_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            todo!()
        }

        fn serialize_struct_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            todo!()
        }
    }

    impl<'a> SerializeSeq for &'a mut FirestoreValueSerializer {
        type Ok = ();

        type Error = Error;

        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }

    impl<'a> SerializeTuple for &'a mut FirestoreValueSerializer {
        type Ok = ();

        type Error = Error;

        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }

    impl<'a> SerializeTupleStruct for &'a mut FirestoreValueSerializer {
        type Ok = ();

        type Error = Error;

        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }

    impl<'a> SerializeTupleVariant for &'a mut FirestoreValueSerializer {
        type Ok = ();

        type Error = Error;

        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }

    impl<'a> SerializeMap for &'a mut FirestoreValueSerializer {
        type Ok = ();

        type Error = Error;

        fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }

    impl<'a> SerializeStruct for &'a mut FirestoreValueSerializer {
        type Ok = ();

        type Error = Error;

        fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }

    impl<'a> SerializeStructVariant for &'a mut FirestoreValueSerializer {
        type Ok = ();

        type Error = Error;

        fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            todo!()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            todo!()
        }
    }

    fn to_value<T>(value: &T) -> Result<Value, Error>
    where
        T: Serialize,
    {
        let mut serializer = FirestoreValueSerializer {
            output: Value::default(),
        };
        value.serialize(&mut serializer)?;
        Ok(serializer.output)
    }

    #[test]
    fn test_bool() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&false)?,
            Value {
                value_type: Some(ValueType::BooleanValue(false))
            }
        );
        assert_eq!(
            to_value(&true)?,
            Value {
                value_type: Some(ValueType::BooleanValue(true))
            }
        );
        Ok(())
    }

    #[test]
    fn test_i8() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&i8::MAX)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::from(i8::MAX)))
            }
        );
        assert_eq!(
            to_value(&i8::MIN)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::from(i8::MIN)))
            }
        );
        Ok(())
    }

    #[test]
    fn test_i16() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&i16::MAX)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::from(i16::MAX)))
            }
        );
        assert_eq!(
            to_value(&i16::MIN)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::from(i16::MIN)))
            }
        );
        Ok(())
    }

    #[test]
    fn test_i32() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&i32::MAX)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::from(i32::MAX)))
            }
        );
        assert_eq!(
            to_value(&i32::MIN)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::from(i32::MIN)))
            }
        );
        Ok(())
    }

    #[test]
    fn test_i64() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&i64::MAX)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::MAX))
            }
        );
        assert_eq!(
            to_value(&i64::MIN)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::MIN))
            }
        );
        Ok(())
    }

    #[test]
    fn test_f32() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&f32::MAX)?,
            Value {
                value_type: Some(ValueType::DoubleValue(f64::from(f32::MAX)))
            }
        );
        assert_eq!(
            to_value(&f32::MIN)?,
            Value {
                value_type: Some(ValueType::DoubleValue(f64::from(f32::MIN)))
            }
        );
        Ok(())
    }

    #[test]
    fn test_f64() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&f64::MAX)?,
            Value {
                value_type: Some(ValueType::DoubleValue(f64::MAX))
            }
        );
        assert_eq!(
            to_value(&f64::MIN)?,
            Value {
                value_type: Some(ValueType::DoubleValue(f64::MIN))
            }
        );
        Ok(())
    }

    #[test]
    fn test_str() -> anyhow::Result<()> {
        // TODO: length check
        assert_eq!(
            to_value(&"abc")?,
            Value {
                value_type: Some(ValueType::StringValue("abc".to_string()))
            }
        );
        Ok(())
    }
}
