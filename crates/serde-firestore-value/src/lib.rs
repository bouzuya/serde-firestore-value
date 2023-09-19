#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fmt::Display};

    use google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};
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
        type Ok = &'a mut FirestoreValueSerializer;

        type Error = Error;

        type SerializeSeq = FirestoreArrayValueSerializer<'a>;

        type SerializeTuple = FirestoreArrayValueSerializer<'a>;

        type SerializeTupleStruct = FirestoreArrayValueSerializer<'a>;

        type SerializeTupleVariant = FirestoreNamedArrayValueSerializer<'a>;

        type SerializeMap = FirestoreMapValueSerializer<'a>;

        type SerializeStruct = Self;

        type SerializeStructVariant = Self;

        fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
            self.output.value_type = Some(ValueType::BooleanValue(v));
            Ok(self)
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
            Ok(self)
        }

        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
            self.serialize_i64(i64::from(v))
        }

        fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
            self.serialize_i64(i64::from(v))
        }

        fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
            self.serialize_i64(i64::from(v))
        }

        fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
            self.serialize_f64(f64::from(v))
        }

        fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
            self.output.value_type = Some(ValueType::DoubleValue(v));
            Ok(self)
        }

        fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
            self.serialize_str(&v.to_string())
        }

        fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
            // TODO: length check
            self.output.value_type = Some(ValueType::StringValue(v.to_string()));
            Ok(self)
        }

        fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
            todo!()
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            self.output.value_type = Some(ValueType::NullValue(0));
            Ok(self)
        }

        fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            value.serialize(self)
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            self.serialize_none()
        }

        fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
            self.serialize_none()
        }

        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            self.serialize_str(variant)
        }

        fn serialize_newtype_struct<T: ?Sized>(
            self,
            _name: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            value.serialize(self)
        }

        fn serialize_newtype_variant<T: ?Sized>(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            let mut map = self.serialize_map(Some(1))?;
            map.serialize_entry(variant, value)?;
            map.end()
        }

        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Ok(FirestoreArrayValueSerializer {
                parent: self,
                output: ArrayValue {
                    values: Vec::with_capacity(len.unwrap_or(0)),
                },
            })
        }

        fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
            self.serialize_seq(Some(len))
        }

        fn serialize_tuple_struct(
            self,
            _name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            self.serialize_tuple(len)
        }

        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            Ok(FirestoreNamedArrayValueSerializer {
                name: variant,
                parent: self,
                output: ArrayValue {
                    values: Vec::with_capacity(len),
                },
            })
        }

        fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(FirestoreMapValueSerializer {
                key: None,
                output: MapValue {
                    fields: HashMap::with_capacity(len.unwrap_or(0)),
                },
                parent: self,
            })
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

    struct FirestoreArrayValueSerializer<'a> {
        output: ArrayValue,
        parent: &'a mut FirestoreValueSerializer,
    }

    impl<'a> SerializeSeq for FirestoreArrayValueSerializer<'a> {
        type Ok = &'a mut FirestoreValueSerializer;

        type Error = Error;

        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            self.output.values.push(to_value(&value)?);
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.parent.output.value_type = Some(ValueType::ArrayValue(self.output));
            Ok(self.parent)
        }
    }

    impl<'a> SerializeTuple for FirestoreArrayValueSerializer<'a> {
        type Ok = &'a mut FirestoreValueSerializer;

        type Error = Error;

        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            self.output.values.push(to_value(&value)?);
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.parent.output.value_type = Some(ValueType::ArrayValue(self.output));
            Ok(self.parent)
        }
    }

    impl<'a> SerializeTupleStruct for FirestoreArrayValueSerializer<'a> {
        type Ok = &'a mut FirestoreValueSerializer;

        type Error = Error;

        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            self.output.values.push(to_value(&value)?);
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.parent.output.value_type = Some(ValueType::ArrayValue(self.output));
            Ok(self.parent)
        }
    }

    struct FirestoreNamedArrayValueSerializer<'a> {
        name: &'static str,
        output: ArrayValue,
        parent: &'a mut FirestoreValueSerializer,
    }

    impl<'a> SerializeTupleVariant for FirestoreNamedArrayValueSerializer<'a> {
        type Ok = &'a mut FirestoreValueSerializer;

        type Error = Error;

        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            self.output.values.push(to_value(&value)?);
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.parent.output.value_type = Some(ValueType::MapValue(MapValue {
                fields: {
                    let mut map = HashMap::new();
                    map.insert(
                        self.name.to_string(),
                        Value {
                            value_type: Some(ValueType::ArrayValue(self.output)),
                        },
                    );
                    map
                },
            }));
            Ok(self.parent)
        }
    }

    struct FirestoreMapValueSerializer<'a> {
        key: Option<String>,
        output: MapValue,
        parent: &'a mut FirestoreValueSerializer,
    }

    impl<'a> SerializeMap for FirestoreMapValueSerializer<'a> {
        type Ok = &'a mut FirestoreValueSerializer;

        type Error = Error;

        fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            if let (
                Value {
                    value_type: Some(ValueType::StringValue(k)),
                },
                None,
            ) = (to_value(&key)?, self.key.as_ref())
            {
                self.key = Some(k);
                Ok(())
            } else {
                todo!()
            }
        }

        fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
        {
            if let Some(k) = self.key.take() {
                let v = to_value(&value)?;
                self.output.fields.insert(k, v);
                Ok(())
            } else {
                todo!()
            }
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.parent.output.value_type = Some(ValueType::MapValue(self.output));
            Ok(self.parent)
        }
    }

    impl<'a> SerializeStruct for &'a mut FirestoreValueSerializer {
        type Ok = &'a mut FirestoreValueSerializer;

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
        type Ok = &'a mut FirestoreValueSerializer;

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
    fn test_u8() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&u8::MAX)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u8::MAX)))
            }
        );
        assert_eq!(
            to_value(&u8::MIN)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u8::MIN)))
            }
        );
        Ok(())
    }

    #[test]
    fn test_u16() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&u16::MAX)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u16::MAX)))
            }
        );
        assert_eq!(
            to_value(&u16::MIN)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u16::MIN)))
            }
        );
        Ok(())
    }

    #[test]
    fn test_u32() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&u32::MAX)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u32::MAX)))
            }
        );
        assert_eq!(
            to_value(&u32::MIN)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u32::MIN)))
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
    fn test_char() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&'a')?,
            Value {
                value_type: Some(ValueType::StringValue("a".to_string()))
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

    #[test]
    fn test_none() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&None::<Option<i64>>)?,
            Value {
                value_type: Some(ValueType::NullValue(0_i32))
            }
        );
        Ok(())
    }

    #[test]
    fn test_some() -> anyhow::Result<()> {
        // TODO: all types
        assert_eq!(
            to_value(&Some(true))?,
            Value {
                value_type: Some(ValueType::BooleanValue(true))
            }
        );
        assert_eq!(
            to_value(&Some(1_i64))?,
            Value {
                value_type: Some(ValueType::IntegerValue(1_i64))
            }
        );
        Ok(())
    }

    #[test]
    fn test_unit() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&())?,
            Value {
                value_type: Some(ValueType::NullValue(0_i32))
            }
        );
        Ok(())
    }

    #[test]
    fn test_unit_struct() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        struct Unit;
        assert_eq!(
            to_value(&Unit)?,
            Value {
                value_type: Some(ValueType::NullValue(0_i32))
            }
        );
        Ok(())
    }

    #[test]
    fn test_unit_variant() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        enum E {
            A,
            B,
        }
        assert_eq!(
            to_value(&E::A)?,
            Value {
                value_type: Some(ValueType::StringValue("A".to_string()))
            }
        );
        assert_eq!(
            to_value(&E::B)?,
            Value {
                value_type: Some(ValueType::StringValue("B".to_string()))
            }
        );
        Ok(())
    }

    #[test]
    fn test_newtype_struct() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        struct Millimeters(u8);
        assert_eq!(
            to_value(&Millimeters(u8::MAX))?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::from(u8::MAX)))
            }
        );
        Ok(())
    }

    #[test]
    fn test_newtype_variant() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        enum E {
            N(u8),
        }
        assert_eq!(
            to_value(&E::N(u8::MAX))?,
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: {
                        let mut map = HashMap::new();
                        map.insert(
                            "N".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(i64::from(u8::MAX))),
                            },
                        );
                        map
                    }
                }))
            }
        );
        Ok(())
    }

    #[test]
    fn test_seq() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&vec![1, 2, 3])?,
            Value {
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: vec![
                        Value {
                            value_type: Some(ValueType::IntegerValue(1))
                        },
                        Value {
                            value_type: Some(ValueType::IntegerValue(2))
                        },
                        Value {
                            value_type: Some(ValueType::IntegerValue(3))
                        }
                    ]
                }))
            }
        );
        assert_eq!(
            to_value(&vec![vec![1]])?,
            Value {
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: vec![Value {
                        value_type: Some(ValueType::ArrayValue(ArrayValue {
                            values: vec![Value {
                                value_type: Some(ValueType::IntegerValue(1))
                            }]
                        }))
                    }]
                }))
            }
        );
        Ok(())
    }

    #[test]
    fn test_tuple() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&(true, 1, "abc"))?,
            Value {
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: vec![
                        Value {
                            value_type: Some(ValueType::BooleanValue(true))
                        },
                        Value {
                            value_type: Some(ValueType::IntegerValue(1))
                        },
                        Value {
                            value_type: Some(ValueType::StringValue("abc".to_string()))
                        }
                    ]
                }))
            }
        );
        Ok(())
    }

    #[test]
    fn test_tuple_struct() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        struct Rgb(u8, u8, u8);
        assert_eq!(
            to_value(&Rgb(1, 2, 3))?,
            Value {
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
                        }
                    ]
                }))
            }
        );
        Ok(())
    }

    #[test]
    fn test_tuple_variant() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        enum E {
            T(u8, u8),
        }
        assert_eq!(
            to_value(&E::T(1, 2))?,
            Value {
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
            }
        );
        Ok(())
    }

    #[test]
    fn test_map() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&{
                let mut map = HashMap::new();
                map.insert("k1", 1_i64);
                map.insert("k2", 2_i64);
                map
            })?,
            Value {
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
            }
        );
        Ok(())
    }
}
