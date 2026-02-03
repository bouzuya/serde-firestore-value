mod firestore_array_value_serializer;
mod firestore_geo_point_value_serializer;
mod firestore_map_value_serializer;
mod firestore_reference_value_serializer;
mod firestore_timestamp_value_serializer;
mod firestore_value_serializer;
mod firestore_value_struct_serializer;
mod name_map_value_serializer;
pub(crate) mod with;

use serde::Serialize;

use crate::google::firestore::v1::Value;
use crate::{Error, ser::firestore_value_serializer::FirestoreValueSerializer};

pub use firestore_value_serializer::FirestoreValueSerializer as Serializer;

/// Serialize an instance of type `T` to a Firestore Value.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// #     use serde_firestore_value::google::firestore::v1::{value::ValueType, MapValue, Value};
/// #     use serde_firestore_value::to_value;
/// #[derive(serde::Serialize)]
/// struct T {
///     b: bool,
///     n: i64,
/// }
/// assert_eq!(
///     to_value(&T { b: true, n: 1 })?,
///     Value {
///         value_type: Some(ValueType::MapValue(MapValue {
///             fields: std::collections::HashMap::from([
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
///     }
/// );
/// #     Ok(())
/// # }
/// ```
///
/// # Serialize GeoPoint, Reference, and Timestamp
///
/// See: [`with`](crate::with).
///
/// # Mapping table
///
/// | [serde data model]         | [Firestore Value]                   |
/// |----------------------------|-------------------------------------|
/// | bool                       | booleanValue                        |
/// | i8                         | integerValue                        |
/// | i16                        | integerValue                        |
/// | i32                        | integerValue                        |
/// | i64                        | integerValue                        |
/// | i128                       | (not supported)                     |
/// | u8                         | integerValue                        |
/// | u16                        | integerValue                        |
/// | u32                        | integerValue                        |
/// | u64                        | (not supported)                     |
/// | u128                       | (not supported)                     |
/// | f32                        | doubleValue                         |
/// | f64                        | doubleValue                         |
/// | char                       | stringValue                         |
/// | string                     | stringValue                         |
/// | byte array                 | bytesValue                          |
/// | option                     | nullValue or (value)                |
/// | unit                       | nullValue                           |
/// | unit_struct                | nullValue                           |
/// | unit_variant               | stringValue                         |
/// | newtype_struct             | (value)                             |
/// | newtype_struct (reference) | referenceValue                      |
/// | newtype_variant            | mapValue (`{ (name): (value) }`)    |
/// | seq                        | arrayValue                          |
/// | tuple                      | arrayValue                          |
/// | tuple_struct               | arrayValue                          |
/// | tuple_variant              | mapValue (`{ (name): arrayValue }`) |
/// | map                        | mapValue (`{ (key): (value) }`)     |
/// | struct                     | mapValue (`{ (field): (value) }`)   |
/// | struct (lat_lng)           | geoPointValue                       |
/// | struct (timestamp)         | timestampValue                      |
/// | struct_variant             | mapValue (`{ (name): mapValue }`)   |
///
/// [Firestore Value]: https://firebase.google.com/docs/firestore/reference/rest/v1/Value
/// [serde data model]: https://serde.rs/data-model.html
pub fn to_value<T>(value: &T) -> Result<Value, Error>
where
    T: Serialize,
{
    value.serialize(FirestoreValueSerializer)
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    const MAX_BYTE_LEN: usize = 1_048_487;

    use crate::value_ext::ValueExt;

    use super::*;

    #[test]
    fn test_bool() -> anyhow::Result<()> {
        assert_eq!(to_value(&false)?, Value::from_bool(false));
        assert_eq!(to_value(&true)?, Value::from_bool(true));
        Ok(())
    }

    #[test]
    fn test_i8() -> anyhow::Result<()> {
        assert_eq!(to_value(&i8::MAX)?, Value::from_i64(i64::from(i8::MAX)));
        assert_eq!(to_value(&i8::MIN)?, Value::from_i64(i64::from(i8::MIN)));
        Ok(())
    }

    #[test]
    fn test_i16() -> anyhow::Result<()> {
        assert_eq!(to_value(&i16::MAX)?, Value::from_i64(i64::from(i16::MAX)));
        assert_eq!(to_value(&i16::MIN)?, Value::from_i64(i64::from(i16::MIN)));
        Ok(())
    }

    #[test]
    fn test_i32() -> anyhow::Result<()> {
        assert_eq!(to_value(&i32::MAX)?, Value::from_i64(i64::from(i32::MAX)));
        assert_eq!(to_value(&i32::MIN)?, Value::from_i64(i64::from(i32::MIN)));
        Ok(())
    }

    #[test]
    fn test_i64() -> anyhow::Result<()> {
        assert_eq!(to_value(&i64::MAX)?, Value::from_i64(i64::MAX));
        assert_eq!(to_value(&i64::MIN)?, Value::from_i64(i64::MIN));
        Ok(())
    }

    #[test]
    fn test_u8() -> anyhow::Result<()> {
        assert_eq!(to_value(&u8::MAX)?, Value::from_i64(i64::from(u8::MAX)));
        assert_eq!(to_value(&u8::MIN)?, Value::from_i64(i64::from(u8::MIN)));
        Ok(())
    }

    #[test]
    fn test_u16() -> anyhow::Result<()> {
        assert_eq!(to_value(&u16::MAX)?, Value::from_i64(i64::from(u16::MAX)));
        assert_eq!(to_value(&u16::MIN)?, Value::from_i64(i64::from(u16::MIN)));
        Ok(())
    }

    #[test]
    fn test_u32() -> anyhow::Result<()> {
        assert_eq!(to_value(&u32::MAX)?, Value::from_i64(i64::from(u32::MAX)));
        assert_eq!(to_value(&u32::MIN)?, Value::from_i64(i64::from(u32::MIN)));
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
        assert_eq!(to_value(&f32::MAX)?, Value::from_f64(f64::from(f32::MAX)));
        assert_eq!(to_value(&f32::MIN)?, Value::from_f64(f64::from(f32::MIN)));
        Ok(())
    }

    #[test]
    fn test_f64() -> anyhow::Result<()> {
        assert_eq!(to_value(&f64::MAX)?, Value::from_f64(f64::MAX));
        assert_eq!(to_value(&f64::MIN)?, Value::from_f64(f64::MIN));
        Ok(())
    }

    #[test]
    fn test_char() -> anyhow::Result<()> {
        assert_eq!(to_value(&'a')?, Value::from_string("a".to_string()));
        Ok(())
    }

    #[test]
    fn test_str() -> anyhow::Result<()> {
        assert_eq!(to_value(&"abc")?, Value::from_string("abc".to_string()));
        assert_eq!(
            to_value(&"a".repeat(MAX_BYTE_LEN))?,
            Value::from_string("a".repeat(MAX_BYTE_LEN))
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
            // ArrayValue is used instead of BytesArray.
            // See: <https://serde.rs/impl-serialize.html#other-special-cases>
            Value::from_values(vec![Value::from_i64(0_i64), Value::from_i64(1_i64)])
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
        assert_eq!(to_value(&None::<Option<i64>>)?, Value::null());
        Ok(())
    }

    #[test]
    fn test_some() -> anyhow::Result<()> {
        assert_eq!(to_value(&Some(true))?, Value::from_bool(true));
        assert_eq!(to_value(&Some(1_i64))?, Value::from_i64(1_i64));
        Ok(())
    }

    #[test]
    fn test_unit() -> anyhow::Result<()> {
        assert_eq!(to_value(&())?, Value::null());
        Ok(())
    }

    #[test]
    fn test_unit_struct() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        struct Unit;
        assert_eq!(to_value(&Unit)?, Value::null());
        Ok(())
    }

    #[test]
    fn test_unit_variant() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        enum E {
            A,
            B,
        }
        assert_eq!(to_value(&E::A)?, Value::from_string("A".to_string()));
        assert_eq!(to_value(&E::B)?, Value::from_string("B".to_string()));
        Ok(())
    }

    #[test]
    fn test_newtype_struct() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        struct Millimeters(u8);
        assert_eq!(
            to_value(&Millimeters(u8::MAX))?,
            Value::from_i64(i64::from(u8::MAX))
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
            Value::from_fields([("N", Value::from_i64(i64::from(u8::MAX)))])
        );
        Ok(())
    }

    #[test]
    fn test_seq() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&vec![1_i64, 2_i64, 3_i64])?,
            Value::from_values(vec![
                Value::from_i64(1_i64),
                Value::from_i64(2_i64),
                Value::from_i64(3_i64)
            ])
        );
        assert_eq!(
            to_value(&vec![vec![1_i64]])?,
            Value::from_values(vec![Value::from_values(vec![Value::from_i64(1_i64)])])
        );
        Ok(())
    }

    #[test]
    fn test_tuple() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&(true, 1, "abc"))?,
            Value::from_values(vec![
                Value::from_bool(true),
                Value::from_i64(1),
                Value::from_string("abc".to_string())
            ])
        );
        Ok(())
    }

    #[test]
    fn test_tuple_struct() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        struct Rgb(u8, u8, u8);
        assert_eq!(
            to_value(&Rgb(1, 2, 3))?,
            Value::from_values(vec![
                Value::from_i64(1),
                Value::from_i64(2),
                Value::from_i64(3)
            ])
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
            Value::from_fields([(
                "T",
                Value::from_values(vec![Value::from_i64(1), Value::from_i64(2)]),
            )])
        );
        Ok(())
    }

    #[test]
    fn test_map() -> anyhow::Result<()> {
        assert_eq!(
            to_value(&{
                let mut map = std::collections::HashMap::new();
                map.insert("k1", 1_i64);
                map.insert("k2", 2_i64);
                map
            })?,
            Value::from_fields([("k1", Value::from_i64(1)), ("k2", Value::from_i64(2))])
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
            Value::from_fields([
                ("r", Value::from_i64(1)),
                ("g", Value::from_i64(2)),
                ("b", Value::from_i64(3)),
            ])
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
            Value::from_fields([(
                "S",
                Value::from_fields([
                    ("r", Value::from_i64(1)),
                    ("g", Value::from_i64(2)),
                    ("b", Value::from_i64(3)),
                ]),
            )])
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
                let mut map = BTreeMap::new();
                map.insert('a', 1_u8);
                map
            })?,
            Value::from_fields([("a", Value::from_i64(1))])
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
