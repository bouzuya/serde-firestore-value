# serde-firestore-value

A serde (de)serializer using Firestore Value as its data format.

[![ci](https://github.com/bouzuya/serde-firestore-value/workflows/ci/badge.svg)](https://github.com/bouzuya/serde-firestore-value/actions)
[![crates.io](https://img.shields.io/crates/v/serde-firestore-value)](https://crates.io/crates/serde-firestore-value)
[![docs.rs](https://img.shields.io/docsrs/serde-firestore-value)](https://docs.rs/serde-firestore-value)
![license](https://img.shields.io/crates/l/serde-firestore-value)

```rust
use serde_firestore_value::FieldReference;
use serde_firestore_value::Function;
use serde_firestore_value::LatLng;
use serde_firestore_value::Pipeline;
use serde_firestore_value::Reference;
use serde_firestore_value::Stage;
use serde_firestore_value::Timestamp;
use serde_firestore_value::google;

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
    // You can use `btree-map` feature instead of `hash-map` feature.
    m: std::collections::HashMap<String, bool>,
    fr: FieldReference,
    f: Function,
    p: Pipeline,
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
        let mut m = std::collections::HashMap::new();
        m.insert("a".to_string(), false);
        m.insert("b".to_string(), true);
        m
    },
    fr: FieldReference("field_name".to_string()),
    f: Function {
        name: "add".to_owned(),
        args: vec![
            google::firestore::v1::Value {
                value_type: Some(google::firestore::v1::value::ValueType::IntegerValue(1)),
            },
            google::firestore::v1::Value {
                value_type: Some(google::firestore::v1::value::ValueType::IntegerValue(2)),
            },
        ],
        options: std::collections::HashMap::new(),
    },
    p: Pipeline {
        stages: vec![Stage {
            name: "filter".to_owned(),
            args: vec![google::firestore::v1::Value {
                value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                    "active = true".to_owned(),
                )),
            }],
            options: std::collections::HashMap::new(),
        }],
    },
};
let value = google::firestore::v1::Value {
    value_type: Some(google::firestore::v1::value::ValueType::MapValue(
        google::firestore::v1::MapValue {
            fields: {
                let mut fields = std::collections::HashMap::new();
                fields.insert(
                    "b".to_string(),
                    google::firestore::v1::Value {
                        value_type: Some(
                            google::firestore::v1::value::ValueType::BooleanValue(true),
                        ),
                    },
                );
                fields.insert(
                    "i".to_string(),
                    google::firestore::v1::Value {
                        value_type: Some(
                            google::firestore::v1::value::ValueType::IntegerValue(1),
                        ),
                    },
                );
                fields.insert(
                    "d".to_string(),
                    google::firestore::v1::Value {
                        value_type: Some(google::firestore::v1::value::ValueType::DoubleValue(
                            2_f64,
                        )),
                    },
                );
                fields.insert(
                    "t".to_string(),
                    google::firestore::v1::Value {
                        value_type: Some(
                            google::firestore::v1::value::ValueType::TimestampValue(
                                prost_types::Timestamp {
                                    seconds: 3_i64,
                                    nanos: 4_i32,
                                },
                            ),
                        ),
                    },
                );
                fields.insert(
                    "s".to_string(),
                    google::firestore::v1::Value {
                        value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                            "s".to_string(),
                        )),
                    },
                );
                fields.insert(
                    "r".to_string(),
                    google::firestore::v1::Value {
                        value_type: Some(
                            google::firestore::v1::value::ValueType::ReferenceValue(
                                "projects/p/databases/d/documents/n".to_string(),
                            ),
                        ),
                    },
                );
                fields.insert(
                    "g".to_string(),
                    google::firestore::v1::Value {
                        value_type: Some(
                            google::firestore::v1::value::ValueType::GeoPointValue(
                                googleapis_tonic_google_firestore_v1::google::r#type::LatLng {
                                    latitude: 5_f64,
                                    longitude: 6_f64,
                                },
                            ),
                        ),
                    },
                );
                fields.insert(
                "a".to_string(),
                google::firestore::v1::Value {
                    value_type: Some(google::firestore::v1::value::ValueType::ArrayValue(google::firestore::v1::ArrayValue {
                        values: vec![
                            google::firestore::v1::Value {
                                value_type: Some(google::firestore::v1::value::ValueType::IntegerValue(1)),
                            },
                            google::firestore::v1::Value {
                                value_type: Some(google::firestore::v1::value::ValueType::IntegerValue(2)),
                            },
                            google::firestore::v1::Value {
                                value_type: Some(google::firestore::v1::value::ValueType::NullValue(0)),
                            },
                        ],
                    })),
                },
            );
                fields.insert(
                "m".to_string(),
                google::firestore::v1::Value {
                    value_type: Some(google::firestore::v1::value::ValueType::MapValue(google::firestore::v1::MapValue {
                        fields: {
                            let mut fields = std::collections::HashMap::new();
                            fields.insert(
                                "a".to_string(),
                                google::firestore::v1::Value {
                                    value_type: Some(google::firestore::v1::value::ValueType::BooleanValue(false)),
                                },
                            );
                            fields.insert(
                                "b".to_string(),
                                google::firestore::v1::Value {
                                    value_type: Some(google::firestore::v1::value::ValueType::BooleanValue(true)),
                                },
                            );
                            fields
                        },
                    })),
                },
            );
                fields.insert(
                    "fr".to_string(),
                    google::firestore::v1::Value {
                        value_type: Some(
                            google::firestore::v1::value::ValueType::FieldReferenceValue(
                                "field_name".to_string(),
                            ),
                        ),
                    },
                );
                fields.insert(
                "f".to_string(),
                google::firestore::v1::Value {
                    value_type: Some(google::firestore::v1::value::ValueType::FunctionValue(google::firestore::v1::Function {
                        name: "add".to_owned(),
                        args: vec![
                            google::firestore::v1::Value {
                                value_type: Some(google::firestore::v1::value::ValueType::IntegerValue(1)),
                            },
                            google::firestore::v1::Value {
                                value_type: Some(google::firestore::v1::value::ValueType::IntegerValue(2)),
                            },
                        ],
                        options: std::collections::HashMap::new(),
                    })),
                },
            );
                fields.insert(
            "p".to_string(),
            google::firestore::v1::Value {
                value_type: Some(google::firestore::v1::value::ValueType::PipelineValue(google::firestore::v1::Pipeline {
                    stages: vec![
                        google::firestore::v1::pipeline::Stage {
                            name: "filter".to_owned(),
                            args: vec![google::firestore::v1::Value {
                                value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                                    "active = true".to_owned(),
                                )),
                            }],
                            options: std::collections::HashMap::new(),
                        },
                    ],
                })),
            },
        );
                fields
            },
        },
    )),
};

let serialized = serde_firestore_value::to_value(&t)?;
assert_eq!(serialized, value);

let deserialized = serde_firestore_value::from_value::<T>(&serialized)?;
assert_eq!(deserialized, t);
```
