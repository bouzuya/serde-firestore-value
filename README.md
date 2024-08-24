# serde-firestore-value

A serde (de)serializer using Firestore Value as its data format.

[![ci](https://github.com/bouzuya/serde-firestore-value/workflows/ci/badge.svg)](https://github.com/bouzuya/serde-firestore-value/actions)
[![crates.io](https://img.shields.io/crates/v/serde-firestore-value)](https://crates.io/crates/serde-firestore-value)
[![docs.rs](https://img.shields.io/docsrs/serde-firestore-value)](https://docs.rs/serde-firestore-value)
![license](https://img.shields.io/crates/l/serde-firestore-value)

```rust
use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};
use serde_firestore_value::{LatLng, Reference, Timestamp};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
struct T {
    b: bool,
    i: i64,
    d: f64,
    t: Timestamp,
    s: String,
    r: Reference,
    g: LatLng,
    a: Vec<Option<i64>>,
    m: BTreeMap<String, bool>,
}

let t = T {
    b: true,
    i: 1_i64,
    d: 2_f64,
    t: Timestamp {
        seconds: 3_i64,
        nanos: 4_i32,
    },
    s: "s".to_string(),
    r: Reference("projects/p/databases/d/documents/n".to_string()),
    g: LatLng {
        latitude: 5_f64,
        longitude: 6_f64,
    },
    a: vec![Some(1), Some(2), None],
    m: {
        let mut m = BTreeMap::new();
        m.insert("a".to_string(), false);
        m.insert("b".to_string(), true);
        m
    },
};
let value = Value {
    value_type: Some(ValueType::MapValue(MapValue {
        fields: {
            let mut fields = std::collections::BTreeMap::new();
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
                "d".to_string(),
                Value {
                    value_type: Some(ValueType::DoubleValue(2_f64)),
                },
            );
            fields.insert(
                "t".to_string(),
                Value {
                    value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
                        seconds: 3_i64,
                        nanos: 4_i32,
                    })),
                },
            );
            fields.insert(
                "s".to_string(),
                Value {
                    value_type: Some(ValueType::StringValue("s".to_string())),
                },
            );
            fields.insert(
                "r".to_string(),
                Value {
                    value_type: Some(ValueType::ReferenceValue(
                        "projects/p/databases/d/documents/n".to_string(),
                    )),
                },
            );
            fields.insert(
                "g".to_string(),
                Value {
                    value_type: Some(ValueType::GeoPointValue(
                        googleapis_tonic_google_firestore_v1::google::r#type::LatLng {
                            latitude: 5_f64,
                            longitude: 6_f64,
                        },
                    )),
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
                            let mut fields = std::collections::BTreeMap::new();
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
```

## Version matrices

| serde-firestore-value | [google-api-proto] | tonic     |
|-----------------------|--------------------|-----------|
| <0.6.0                | (unknown)          | (unknown) |
| >=0.6.0               | >=1.516.0          | 0.11.x    |

[google-api-proto]: https://github.com/mechiru/google-api-proto
