#[cfg(feature = "hash-map")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        MapValue, Value, value::ValueType,
    };
    use serde_firestore_value::from_value;

    #[allow(dead_code)]
    #[derive(Debug, serde::Deserialize)]
    enum T {
        A(i64),
    }

    assert_eq!(
        from_value::<'_, T>(&Value {
            value_type: Some(ValueType::MapValue(MapValue {
                fields: {
                    let mut fields = std::collections::HashMap::new();
                    fields.insert(
                        "B".to_string(),
                        Value {
                            value_type: Some(ValueType::IntegerValue(2)),
                        },
                    );
                    fields
                }
            },))
        })
        .unwrap_err()
        .to_string(),
        "unknown variant `B`, expected `A`"
    );
    assert_eq!(
        from_value::<'_, T>(&Value {
            value_type: Some(ValueType::MapValue(MapValue {
                fields: {
                    let mut fields = std::collections::HashMap::new();
                    fields.insert(
                        "A".to_string(),
                        Value {
                            value_type: Some(ValueType::IntegerValue(1)),
                        },
                    );
                    fields.insert(
                        "B".to_string(),
                        Value {
                            value_type: Some(ValueType::IntegerValue(2)),
                        },
                    );
                    fields
                }
            },))
        })
        .unwrap_err()
        .to_string(),
        "invalid length 2, expected 1"
    );

    // FYI
    assert!(
        serde_json::from_str::<'_, T>(r#"{"B":2}"#)
            .unwrap_err()
            .to_string()
            .starts_with("unknown variant `B`, expected `A`")
    );
    assert!(
        serde_json::from_str::<'_, T>(r#"{"B":2,"A":1}"#)
            .unwrap_err()
            .to_string()
            .starts_with("unknown variant `B`, expected `A`")
    );
    assert!(
        serde_json::from_str::<'_, T>(r#"{"A":1,"B":2}"#)
            .unwrap_err()
            .to_string()
            .starts_with("expected value at ")
    );
    Ok(())
}
