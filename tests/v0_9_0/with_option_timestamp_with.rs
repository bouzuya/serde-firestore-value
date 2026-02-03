#[cfg(feature = "hash-map")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        MapValue, Value, value::ValueType,
    };
    use serde_firestore_value::{from_value, to_value, with::option_timestamp};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S {
        #[serde(with = "option_timestamp")]
        a: Option<prost_types::Timestamp>,
        #[serde(with = "option_timestamp")]
        b: Option<prost_types::Timestamp>,
    }
    let o = S {
        a: Some(prost_types::Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        }),
        b: None,
    };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    "a".to_string(),
                    Value {
                        value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
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
