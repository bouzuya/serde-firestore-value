mod error;
mod firestore_array_value_serializer;
mod firestore_map_value_serializer;
mod firestore_value_serializer;

use google::firestore::v1::Value;
use serde::Serialize;

use crate::serializer::firestore_value_serializer::FirestoreValueSerializer;

pub use self::error::Error;

pub fn to_value<T>(value: &T) -> Result<Value, Error>
where
    T: Serialize,
{
    let mut serializer = FirestoreValueSerializer::default();
    value.serialize(&mut serializer)?;
    Ok(serializer.into_inner())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use google::firestore::v1::{value::ValueType, ArrayValue, MapValue};

    const MAX_BYTE_LEN: usize = 1_048_487;

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
            "u64 is not supported"
        );
        assert_eq!(
            to_value(&u64::MIN).unwrap_err().to_string(),
            "u64 is not supported"
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
