use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, MapValue, Value};
use prost_types::Timestamp;
use serde_firestore_value::{from_value, to_value, with::option_timestamp};

#[test]
fn test_deserialize_with() -> anyhow::Result<()> {
    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct S(#[serde(deserialize_with = "option_timestamp::deserialize")] Option<Timestamp>);

    // some
    {
        let o = S(Some(Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        }));
        let v = Value {
            value_type: Some(ValueType::TimestampValue(Timestamp {
                seconds: 1_i64,
                nanos: 2_i32,
            })),
        };
        let d = from_value::<'_, S>(&v)?;
        assert_eq!(d, o);
    }

    // none
    {
        let o = S(None);
        let v = Value {
            value_type: Some(ValueType::NullValue(0_i32)),
        };
        let d = from_value::<'_, S>(&v)?;
        assert_eq!(d, o);
    }
    Ok(())
}

#[test]
fn test_serialize_with() -> anyhow::Result<()> {
    #[derive(Debug, Eq, PartialEq, serde::Serialize)]
    struct S(#[serde(serialize_with = "option_timestamp::serialize")] Option<Timestamp>);

    // some
    {
        let o = S(Some(Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        }));
        let v = Value {
            value_type: Some(ValueType::TimestampValue(Timestamp {
                seconds: 1_i64,
                nanos: 2_i32,
            })),
        };
        let s = to_value(&o)?;
        assert_eq!(s, v);
    }

    // none
    {
        let o = S(None);
        let v = Value {
            value_type: Some(ValueType::NullValue(0)),
        };
        let s = to_value(&o)?;
        assert_eq!(s, v);
    }
    Ok(())
}

#[test]
fn test_struct() -> anyhow::Result<()> {
    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S {
        #[serde(with = "option_timestamp")]
        a: Option<Timestamp>,
        #[serde(with = "option_timestamp")]
        b: Option<Timestamp>,
    }
    let o = S {
        a: Some(Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        }),
        b: None,
    };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut map = HashMap::new();
                map.insert(
                    "a".to_string(),
                    Value {
                        value_type: Some(ValueType::TimestampValue(Timestamp {
                            seconds: 1_i64,
                            nanos: 2_i32,
                        })),
                    },
                );
                map.insert(
                    "b".to_string(),
                    Value {
                        value_type: Some(ValueType::NullValue(0_i32)),
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
