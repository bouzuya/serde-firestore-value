use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};
use prost_types::Timestamp;
use serde_firestore_value::{from_value, to_value};

mod timestamp_as_tuple {
    use prost_types::Timestamp;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde::Deserialize::<'de>::deserialize(deserializer)
            .map(|(seconds, nanos): (i64, i32)| Timestamp { seconds, nanos })
    }

    pub fn serialize<S>(timestamp: &Timestamp, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tuple = serializer.serialize_tuple(2)?;
        serde::ser::SerializeTuple::serialize_element(&mut tuple, &timestamp.seconds)?;
        serde::ser::SerializeTuple::serialize_element(&mut tuple, &timestamp.nanos)?;
        serde::ser::SerializeTuple::end(tuple)
    }
}

#[test]
fn test_deserialize_with() -> anyhow::Result<()> {
    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct S {
        #[serde(deserialize_with = "timestamp_as_tuple::deserialize")]
        a: Timestamp,
    }

    let o = S {
        a: Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        },
    };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut map = HashMap::new();
                map.insert(
                    "a".to_string(),
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
            },
        })),
    };
    assert_eq!(from_value::<'_, S>(&v)?, o);
    Ok(())
}

#[test]
fn test_serialize_with() -> anyhow::Result<()> {
    #[derive(Debug, Eq, PartialEq, serde::Serialize)]
    struct S {
        #[serde(serialize_with = "timestamp_as_tuple::serialize")]
        a: Timestamp,
    }

    let o = S {
        a: Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        },
    };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut map = HashMap::new();
                map.insert(
                    "a".to_string(),
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
            },
        })),
    };
    let s = to_value(&o)?;
    assert_eq!(s, v);
    Ok(())
}

#[test]
fn test_with() -> anyhow::Result<()> {
    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S {
        #[serde(with = "timestamp_as_tuple")]
        a: Timestamp,
    }

    let o = S {
        a: Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        },
    };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut map = HashMap::new();
                map.insert(
                    "a".to_string(),
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
            },
        })),
    };

    let s = to_value(&o)?;
    let d = from_value::<'_, S>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
