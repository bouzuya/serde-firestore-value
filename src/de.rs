mod firestore_array_value_deserializer;
mod firestore_enum_deserializer;
mod firestore_geo_point_value_deserializer;
mod firestore_map_value_deserializer;
mod firestore_reference_value_deserializer;
mod firestore_struct_map_value_deserializer;
mod firestore_timestamp_value_deserializer;
mod firestore_value_deserializer;
pub(crate) mod with;

use crate::Error;

use self::firestore_value_deserializer::FirestoreValueDeserializer;

use google_api_proto::google::firestore::v1::Value;

pub use self::firestore_value_deserializer::FirestoreValueDeserializer as Deserializer;

/// Deserialize an instance of type `T` from a Firestore Value.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// #     use google_api_proto::google::firestore::v1::{value::ValueType, MapValue, Value};
/// #     use serde_firestore_value::from_value;
/// #     use std::collections::BTreeMap;
/// #[derive(Debug, PartialEq, serde::Deserialize)]
/// struct T {
///     b: bool,
///     n: i64,
/// }
/// assert_eq!(
///     from_value::<'_, T>(&Value {
///         value_type: Some(ValueType::MapValue(MapValue {
///             fields: BTreeMap::from([
///                 (
///                     "b".to_string(),
///                     Value {
///                         value_type: Some(ValueType::BooleanValue(true))
///                     }
///                 ),
///                 (
///                     "n".to_string(),
///                     Value {
///                         value_type: Some(ValueType::IntegerValue(1))
///                     }
///                 )
///             ])
///         }))
///     })?,
///     T { b: true, n: 1 }
/// );
/// #     Ok(())
/// # }
/// ```
///
/// # Deserialize GeoPoint, Reference, and Timestamp
///
/// See: [`with`](crate::with).
///
/// # Mapping table (no type hint)
///
/// | [Firestore Value]  | [serde data model]                            |
/// |--------------------|-----------------------------------------------|
/// | nullValue          | unit                                          |
/// | booleanValue       | bool                                          |
/// | integerValue       | i64                                           |
/// | doubleValue        | f64                                           |
/// | timestampValue     | map (`{ "seconds": i64, "nanos": i64 }`)      |
/// | stringValue        | string                                        |
/// | bytesValue         | bytes                                         |
/// | referenceValue     | string                                        |
/// | geoPointValue      | map (`{ "latitude": f64, "longitude": f64 }`) |
/// | arrayValue         | seq                                           |
/// | mapValue           | map                                           |
///
/// [Firestore Value]: https://firebase.google.com/docs/firestore/reference/rest/v1/Value
/// [serde data model]: https://serde.rs/data-model.html
pub fn from_value<'a, T>(value: &'a Value) -> Result<T, Error>
where
    T: serde::Deserialize<'a>,
{
    T::deserialize(FirestoreValueDeserializer::new(value))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use google_api_proto::google::firestore::v1::Value;

    use crate::value_ext::ValueExt;

    use super::*;

    #[test]
    fn test_deserialize_bool() -> anyhow::Result<()> {
        assert!(from_value::<'_, bool>(&Value::from_bool(true))?);
        assert!(!from_value::<'_, bool>(&Value::from_bool(false))?);
        Ok(())
    }

    #[test]
    fn test_deserialize_i8() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, i8>(&Value::from_i64(i64::from(i8::MAX)))?,
            i8::MAX
        );
        assert_eq!(
            from_value::<'_, i8>(&Value::from_i64(i64::from(i8::MIN)))?,
            i8::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_i16() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, i16>(&Value::from_i64(i64::from(i16::MAX)))?,
            i16::MAX
        );
        assert_eq!(
            from_value::<'_, i16>(&Value::from_i64(i64::from(i16::MIN)))?,
            i16::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_i32() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, i32>(&Value::from_i64(i64::from(i32::MAX)))?,
            i32::MAX
        );
        assert_eq!(
            from_value::<'_, i32>(&Value::from_i64(i64::from(i32::MIN)))?,
            i32::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_i64() -> anyhow::Result<()> {
        assert_eq!(from_value::<'_, i64>(&Value::from_i64(i64::MAX))?, i64::MAX);
        assert_eq!(from_value::<'_, i64>(&Value::from_i64(i64::MIN))?, i64::MIN);
        Ok(())
    }

    #[test]
    fn test_deserialize_u8() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, u8>(&Value::from_i64(i64::from(u8::MAX)))?,
            u8::MAX
        );
        assert_eq!(
            from_value::<'_, u8>(&Value::from_i64(i64::from(u8::MIN)))?,
            u8::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_u16() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, u16>(&Value::from_i64(i64::from(u16::MAX)))?,
            u16::MAX
        );
        assert_eq!(
            from_value::<'_, u16>(&Value::from_i64(i64::from(u16::MIN)))?,
            u16::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_u32() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, u32>(&Value::from_i64(i64::from(u32::MAX)))?,
            u32::MAX
        );
        assert_eq!(
            from_value::<'_, u32>(&Value::from_i64(i64::from(u32::MIN)))?,
            u32::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_u64() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, u64>(&Value::from_i64(i64::try_from(u64::MIN)?))
                .unwrap_err()
                .to_string(),
            "u64 is not supported"
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_f32() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, f32>(&Value::from_f64(f64::from(f32::MAX)))?,
            f32::MAX
        );
        assert_eq!(
            from_value::<'_, f32>(&Value::from_f64(f64::from(f32::MIN)))?,
            f32::MIN
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_f64() -> anyhow::Result<()> {
        assert_eq!(from_value::<'_, f64>(&Value::from_f64(f64::MAX))?, f64::MAX);
        assert_eq!(from_value::<'_, f64>(&Value::from_f64(f64::MIN))?, f64::MIN);
        Ok(())
    }

    #[test]
    fn test_deserialize_char() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, char>(&Value::from_string("a".to_string()))?,
            'a'
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_str() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, String>(&Value::from_string("abc".to_string()))?,
            // "abc".to_string()
            "abc"
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_string() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, String>(&Value::from_string("abc".to_string()))?,
            // "abc"
            "abc".to_string()
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_option() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, Option<bool>>(&Value::from_bool(true))?,
            Some(true)
        );
        assert_eq!(from_value::<'_, Option<bool>>(&Value::null())?, None);
        Ok(())
    }

    #[test]
    fn test_deserialize_unit() -> anyhow::Result<()> {
        from_value::<'_, ()>(&Value::null())?;
        Ok(())
    }

    #[test]
    fn test_deserialize_unit_struct() -> anyhow::Result<()> {
        #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
        struct Unit;
        assert_eq!(from_value::<'_, Unit>(&Value::null())?, Unit);
        Ok(())
    }

    #[test]
    fn test_deserialize_newtype_struct() -> anyhow::Result<()> {
        #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
        struct Millimeters(u8);
        assert_eq!(
            from_value::<'_, Millimeters>(&Value::from_i64(i64::from(u8::MAX)))?,
            Millimeters(u8::MAX)
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_seq() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, Vec<i64>>(&Value::from_values(vec![
                Value::from_i64(1_i64),
                Value::from_i64(2_i64),
                Value::from_i64(3_i64),
            ]))?,
            vec![1_i64, 2_i64, 3_i64]
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_tuple() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, (bool, i64)>(&Value::from_values(vec![
                Value::from_bool(true),
                Value::from_i64(1_i64),
            ]))?,
            (true, 1_i64)
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_tuple_struct() -> anyhow::Result<()> {
        #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
        struct Rgb(u8, u8, u8);
        assert_eq!(
            from_value::<'_, Rgb>(&Value::from_values(vec![
                Value::from_i64(1_i64),
                Value::from_i64(2_i64),
                Value::from_i64(3_i64),
            ]))?,
            Rgb(1_u8, 2_u8, 3_u8)
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_map() -> anyhow::Result<()> {
        assert_eq!(
            from_value::<'_, BTreeMap<String, i64>>(&Value::from_fields([
                ("k1", Value::from_i64(1_i64)),
                ("k2", Value::from_i64(2_i64)),
            ]))?,
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
            from_value::<'_, S>(&Value::from_fields([
                ("r", Value::from_i64(1_i64)),
                ("g", Value::from_i64(2_i64)),
                ("b", Value::from_i64(3_i64)),
            ]))?,
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
                from_value::<'_, E>(&Value::from_string("A".to_string()))?,
                E::A
            );
            assert_eq!(
                from_value::<'_, E>(&Value::from_string("B".to_string()))?,
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
                from_value::<'_, E>(&Value::from_fields([("A", Value::from_i64(1_i64)),]))?,
                E::A(1_u8)
            );
            assert_eq!(
                from_value::<'_, E>(&Value::from_fields([("B", Value::from_i64(2_i64)),]))?,
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
                from_value::<'_, E>(&Value::from_fields([(
                    "T",
                    Value::from_values(vec![Value::from_i64(1_i64), Value::from_i64(2_i64)]),
                )]))?,
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
                from_value::<'_, E>(&Value::from_fields([(
                    "S",
                    Value::from_fields({
                        let mut fields = BTreeMap::new();
                        fields.insert("r".to_string(), Value::from_i64(1_i64));
                        fields.insert("g".to_string(), Value::from_i64(2_i64));
                        fields.insert("b".to_string(), Value::from_i64(3_i64));
                        fields
                    }),
                )]))?,
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
            from_value::<'_, bool>(&Value::from_i64(1_i64))
                .unwrap_err()
                .to_string(),
            "invalid type: integer value, expected boolean value"
        );

        // expected integer value
        assert_eq!(
            from_value::<'_, i8>(&Value::from_bool(true))
                .unwrap_err()
                .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        assert_eq!(
            from_value::<'_, i16>(&Value::from_bool(true))
                .unwrap_err()
                .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        assert_eq!(
            from_value::<'_, i32>(&Value::from_bool(true))
                .unwrap_err()
                .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        assert_eq!(
            from_value::<'_, i64>(&Value::from_bool(true))
                .unwrap_err()
                .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        assert_eq!(
            from_value::<'_, u8>(&Value::from_bool(true))
                .unwrap_err()
                .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        assert_eq!(
            from_value::<'_, u16>(&Value::from_bool(true))
                .unwrap_err()
                .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        assert_eq!(
            from_value::<'_, u32>(&Value::from_bool(true))
                .unwrap_err()
                .to_string(),
            "invalid type: boolean value, expected integer value"
        );
        // u64 is not supported

        // expected double value
        assert_eq!(
            from_value::<'_, f32>(&Value::from_bool(true))
                .unwrap_err()
                .to_string(),
            "invalid type: boolean value, expected double value"
        );
        assert_eq!(
            from_value::<'_, f64>(&Value::from_bool(true))
                .unwrap_err()
                .to_string(),
            "invalid type: boolean value, expected double value"
        );

        // TODO: ...
        Ok(())
    }
}
