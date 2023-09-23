mod error;
mod firestore_array_value_deserializer;
mod firestore_enum_deserializer;
mod firestore_map_value_deserializer;
mod firestore_struct_map_value_deserializer;
mod firestore_value_deserializer;
pub mod timestamp;
mod value_ext;
mod value_type_ext;
mod value_type_name;

use self::{
    error::{Error, ErrorCode},
    firestore_value_deserializer::FirestoreValueDeserializer,
    value_type_name::ValueTypeName,
};

use google::firestore::v1::Value;

pub fn from_value<'a, T>(value: &'a Value) -> Result<T, Error>
where
    T: serde::Deserialize<'a>,
{
    T::deserialize(FirestoreValueDeserializer::new(value))
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};

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
