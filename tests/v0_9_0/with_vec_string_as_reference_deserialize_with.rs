#[cfg(feature = "hash-map")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        ArrayValue, MapValue, Value, value::ValueType,
    };
    use serde_firestore_value::{from_value, with::vec_string_as_reference};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct S {
        #[serde(deserialize_with = "vec_string_as_reference::deserialize")]
        r1: Vec<String>,
        #[serde(deserialize_with = "vec_string_as_reference::deserialize")]
        r2: Vec<String>,
        s1: Vec<String>,
        s2: Vec<String>,
    }

    let o = S {
        r1: vec!["projects/p/databases/d/documents/c/1".to_string()],
        r2: vec![],
        s1: vec!["s1".to_string()],
        s2: vec![],
    };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut fields = std::collections::HashMap::new();
                fields.insert(
                    "r1".to_string(),
                    Value {
                        value_type: Some(ValueType::ArrayValue(ArrayValue {
                            values: vec![Value {
                                value_type: Some(ValueType::ReferenceValue(
                                    "projects/p/databases/d/documents/c/1".to_string(),
                                )),
                            }],
                        })),
                    },
                );
                fields.insert(
                    "r2".to_string(),
                    Value {
                        value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![] })),
                    },
                );
                fields.insert(
                    "s1".to_string(),
                    Value {
                        value_type: Some(ValueType::ArrayValue(ArrayValue {
                            values: vec![Value {
                                value_type: Some(ValueType::StringValue("s1".to_string())),
                            }],
                        })),
                    },
                );
                fields.insert(
                    "s2".to_string(),
                    Value {
                        value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![] })),
                    },
                );
                fields
            },
        })),
    };
    let d = from_value::<'_, S>(&v)?;
    assert_eq!(d, o);
    Ok(())
}
