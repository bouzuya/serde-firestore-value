#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        value::ValueType, ArrayValue, MapValue, Value,
    };
    use serde_firestore_value::{from_value, to_value, with::vec_string_as_reference};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S {
        #[serde(with = "vec_string_as_reference")]
        r1: Vec<String>,
        #[serde(with = "vec_string_as_reference")]
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
    let s = to_value(&o)?;
    let d = from_value::<'_, S>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
