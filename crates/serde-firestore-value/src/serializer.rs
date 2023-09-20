use google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};
use serde::{ser::SerializeMap, Serialize, Serializer};

use crate::{
    firestore_array_value_serializer::FirestoreArrayValueSerializer,
    firestore_map_value_serializer::FirestoreMapValueSerializer,
    firestore_named_array_value_serializer::FirestoreNamedArrayValueSerializer,
    firestore_named_map_value_serializer::FirestoreNamedMapValueSerializer, Error, ErrorCode,
};

pub fn to_value<T>(value: &T) -> Result<Value, Error>
where
    T: Serialize,
{
    let mut serializer = FirestoreValueSerializer {
        output: Value::default(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

struct FirestoreValueSerializer {
    pub(crate) output: Value,
}

impl crate::firestore_array_value_serializer::SetArrayValue for FirestoreValueSerializer {
    fn set_array_value(&mut self, value: ArrayValue) {
        self.output.value_type = Some(ValueType::ArrayValue(value));
    }
}

impl crate::firestore_named_array_value_serializer::SetMapValue for FirestoreValueSerializer {
    fn set_map_value(&mut self, value: MapValue) {
        self.output.value_type = Some(ValueType::MapValue(value));
    }
}

// 1,048,487 bytes = 1MiB - 89 bytes
const MAX_BYTE_LEN: usize = 1_048_487;

impl<'a> Serializer for &'a mut FirestoreValueSerializer {
    type Ok = &'a mut FirestoreValueSerializer;

    type Error = Error;

    type SerializeSeq = FirestoreArrayValueSerializer<'a, FirestoreValueSerializer>;

    type SerializeTuple = FirestoreArrayValueSerializer<'a, FirestoreValueSerializer>;

    type SerializeTupleStruct = FirestoreArrayValueSerializer<'a, FirestoreValueSerializer>;

    type SerializeTupleVariant = FirestoreNamedArrayValueSerializer<'a, FirestoreValueSerializer>;

    type SerializeMap = FirestoreMapValueSerializer<'a, FirestoreValueSerializer>;

    type SerializeStruct = FirestoreMapValueSerializer<'a, FirestoreValueSerializer>;

    type SerializeStructVariant = FirestoreNamedMapValueSerializer<'a, FirestoreValueSerializer>;

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
        self.serialize_i64(i64::try_from(v).map_err(|_| Error::from(ErrorCode::IntegerOutOfRange))?)
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
        if v.len() > MAX_BYTE_LEN {
            return Err(Error::from(ErrorCode::MaximumByteLengthExceeded));
        }
        self.output.value_type = Some(ValueType::StringValue(v.to_string()));
        Ok(self)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        // NOTE: unreachable. See: <https://serde.rs/impl-serialize.html#other-special-cases>
        if v.len() > MAX_BYTE_LEN {
            return Err(Error::from(ErrorCode::MaximumByteLengthExceeded));
        }
        self.output.value_type = Some(ValueType::BytesValue(v.to_vec()));
        Ok(self)
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
        SerializeMap::end(map)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(FirestoreArrayValueSerializer::new(self, len))
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
        Ok(FirestoreNamedArrayValueSerializer::new(self, variant, len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(FirestoreMapValueSerializer::new(self, len))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(FirestoreNamedMapValueSerializer::new(self, variant, len))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

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
    fn test_u64() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&u64::MAX).unwrap_err().to_string(),
            "integer out of range"
        );
        let i64_max_as_u64 = u64::try_from(i64::MAX)?;
        assert_eq!(
            to_value(&i64_max_as_u64)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::try_from(i64_max_as_u64)?))
            }
        );
        assert_eq!(
            to_value(&u64::MIN)?,
            Value {
                value_type: Some(ValueType::IntegerValue(i64::try_from(u64::MIN)?))
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
        assert_eq!(
            to_value(&"abc")?,
            Value {
                value_type: Some(ValueType::StringValue("abc".to_string()))
            }
        );
        assert_eq!(
            to_value(&"a".repeat(MAX_BYTE_LEN))?,
            Value {
                value_type: Some(ValueType::StringValue("a".repeat(MAX_BYTE_LEN)))
            }
        );
        assert_eq!(
            to_value(&"a".repeat(MAX_BYTE_LEN + 1))
                .unwrap_err()
                .to_string(),
            "maximum byte length (1,048,487 bytes = 1MiB - 89 bytes) exceeded"
        );

        Ok(())
    }

    #[test]
    fn test_bytes() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&[0_u8, 1_u8])?,
            Value {
                // ArrayValue is used instead of BytesArray.
                // See: <https://serde.rs/impl-serialize.html#other-special-cases>
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: vec![
                        Value {
                            value_type: Some(ValueType::IntegerValue(0_i64))
                        },
                        Value {
                            value_type: Some(ValueType::IntegerValue(1_i64))
                        },
                    ]
                }))
            }
        );
        // ArrayValue length is not checked.
        // assert!(to_value(&vec![0_u8; MAX_BYTE_LEN]).is_ok());
        // assert_eq!(
        //     to_value(&vec![0_u8; MAX_BYTE_LEN + 1])
        //         .unwrap_err()
        //         .to_string(),
        //     "maximum byte length (1,048,487 bytes = 1MiB - 89 bytes) exceeded"
        // );
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

    #[test]
    fn test_struct() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        struct S {
            r: u8,
            g: u8,
            b: u8,
        }
        assert_eq!(
            to_value(&S { r: 1, g: 2, b: 3 })?,
            Value {
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
            }
        );
        Ok(())
    }

    #[test]
    fn test_struct_variant() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        enum E {
            S { r: u8, g: u8, b: u8 },
        }
        assert_eq!(
            to_value(&E::S { r: 1, g: 2, b: 3 })?,
            Value {
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
                                    },
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
    fn test_error_key_must_be_a_string() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&{
                let mut map = HashMap::new();
                map.insert((), 1_u8);
                map
            })
            .unwrap_err()
            .to_string(),
            "key must be a string"
        );
        assert_eq!(
            to_value(&{
                let mut map = HashMap::new();
                map.insert('a', 1_u8);
                map
            })?,
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: {
                        let mut map = HashMap::new();
                        map.insert(
                            "a".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(1_i64)),
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
    fn test_impl_serde_ser_error() {
        fn assert_impl<T: serde::ser::Error>() {}
        assert_impl::<Error>();
        assert_eq!(
            <Error as serde::ser::Error>::custom("custom error").to_string(),
            "custom error"
        );
    }
}
