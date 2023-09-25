# serde-firestore-value

A serde (de)serializer using Firestore Value as data format.

## TODOs

- ☐ Use <https://crates.io/crates/google-api-proto>
- ☐ crates.io
- ☐ docs.rs
- ☐ GitHub Actions

## API Overview

```rust
pub fn from_value<'de, T>(value: &'de Value) -> Result<T, Error>
where
    T: serde::Deserialize<'de>;

pub fn to_value<T>(value: &T) -> Result<Value, Error>
where
    T: serde::Serialize;
```

## Example

```rust
use google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};

#[test]
fn test() -> anyhow::Result<()> {
    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct T {
        b: bool,
        i: i64,
        s: String,
        a: Vec<Option<i64>>,
        m: std::collections::HashMap<String, bool>,
    }

    let t = T {
        b: true,
        i: 1,
        s: "s".to_string(),
        a: vec![Some(1), Some(2), None],
        m: {
            let mut m = std::collections::HashMap::new();
            m.insert("a".to_string(), false);
            m.insert("b".to_string(), true);
            m
        },
    };
    let value = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut fields = std::collections::HashMap::new();
                fields.insert(
                    "b".to_string(),
                    Value {
                        value_type: Some(ValueType::BooleanValue(true)),
                    },
                );
                fields.insert(
                    "i".to_string(),
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                );
                fields.insert(
                    "s".to_string(),
                    Value {
                        value_type: Some(ValueType::StringValue("s".to_string())),
                    },
                );
                fields.insert(
                    "a".to_string(),
                    Value {
                        value_type: Some(ValueType::ArrayValue(ArrayValue {
                            values: vec![
                                Value {
                                    value_type: Some(ValueType::IntegerValue(1)),
                                },
                                Value {
                                    value_type: Some(ValueType::IntegerValue(2)),
                                },
                                Value {
                                    value_type: Some(ValueType::NullValue(0)),
                                },
                            ],
                        })),
                    },
                );
                fields.insert(
                    "m".to_string(),
                    Value {
                        value_type: Some(ValueType::MapValue(MapValue {
                            fields: {
                                let mut fields = std::collections::HashMap::new();
                                fields.insert(
                                    "a".to_string(),
                                    Value {
                                        value_type: Some(ValueType::BooleanValue(false)),
                                    },
                                );
                                fields.insert(
                                    "b".to_string(),
                                    Value {
                                        value_type: Some(ValueType::BooleanValue(true)),
                                    },
                                );
                                fields
                            },
                        })),
                    },
                );
                fields
            },
        })),
    };

    let serialized = serde_firestore_value::to_value(&t)?;
    assert_eq!(serialized, value);

    let deserialized = serde_firestore_value::from_value::<T>(&serialized)?;
    assert_eq!(deserialized, t);

    Ok(())
}
```

## serializer mapping table

| [serde data model]         | [firestore Value]                   |
|----------------------------|-------------------------------------|
| bool                       | booleanValue                        |
| i8                         | integerValue                        |
| i16                        | integerValue                        |
| i32                        | integerValue                        |
| i64                        | integerValue                        |
| i128                       | (not supported)                     |
| u8                         | integerValue                        |
| u16                        | integerValue                        |
| u32                        | integerValue                        |
| u64                        | (not supported)                     |
| u128                       | (not supported)                     |
| f32                        | doubleValue                         |
| f64                        | doubleValue                         |
| char                       | stringValue                         |
| string                     | stringValue                         |
| byte array                 | bytesValue                          |
| option                     | nullValue or (value)                |
| unit                       | nullValue                           |
| unit_struct                | nullValue                           |
| unit_variant               | stringValue                         |
| newtype_struct             | (value)                             |
| newtype_struct (reference) | referenceValue                      |
| newtype_variant            | mapValue (`{ (name): (value) }`)    |
| seq                        | arrayValue                          |
| tuple                      | arrayValue                          |
| tuple_struct               | arrayValue                          |
| tuple_variant              | mapValue (`{ (name): arrayValue }`) |
| map                        | mapValue (`{ (key): (value) }`)     |
| struct                     | mapValue (`{ (field): (value) }`)   |
| struct (lat_lng)           | geoPointValue                       |
| struct (timestamp)         | timestampValue                      |
| struct_variant             | mapValue (`{ (name): mapValue }`)   |

## deserializer mapping table (no type hint)

| [firestore Value]  | [serde data model]                            |
|--------------------|-----------------------------------------------|
| nullValue          | unit                                          |
| booleanValue       | bool                                          |
| integerValue       | i64                                           |
| doubleValue        | f64                                           |
| timestampValue     | map (`{ "seconds": i64, "nanos": i64 }`)      |
| stringValue        | string                                        |
| bytesValue         | bytes                                         |
| referenceValue     | string                                        |
| geoPointValue      | map (`{ "latitude": f64, "longitude": f64 }`) |
| arrayValue         | seq                                           |
| mapValue           | map                                           |

[serde data model]: https://serde.rs/data-model.html
[firestore Value]: https://firebase.google.com/docs/firestore/reference/rest/v1/Value

## NOTE: firestore value types

```rust
use ::prost::alloc::string::String;
use ::prost::alloc::vec::Vec;
use ::std::collections::HashMap;
use ::core::option::Option;
use ::prost_types::Timestamp;
use /* ... */::LatLng;

struct Value {
    value_type: Option<ValueType>,
}

enum ValueType {
    NullValue(i32),
    BooleanValue(bool),
    IntegerValue(i64),
    DoubleValue(f64),
    TimestampValue(Timestamp),
    StringValue(String),
    BytesValue(Vec<u8>),
    ReferenceValue(String),
    GeoPointValue(LatLng),
    ArrayValue(ArrayValue),
    MapValue(MapValue),
}

struct ArrayValue {
    values: Vec<Value>,
}

struct MapValue {
    fields: HashMap<String, Value>,
}

struct LatLng {
    latitude: f64,
    longitude: f64,
}
```
