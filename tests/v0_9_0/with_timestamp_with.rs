#[cfg(feature = "hash-map")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        value::ValueType, MapValue, Value,
    };
    use serde_firestore_value::{from_value, to_value, with::timestamp};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S {
        #[serde(with = "timestamp")]
        a: prost_types::Timestamp,
    }
    let o = S {
        a: prost_types::Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        },
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
