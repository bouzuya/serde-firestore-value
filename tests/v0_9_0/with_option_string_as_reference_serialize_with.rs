#[cfg(feature = "hash-map")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        MapValue, Value, value::ValueType,
    };
    use serde_firestore_value::{to_value, with::option_string_as_reference};

    #[derive(Debug, Eq, PartialEq, serde::Serialize)]
    struct S {
        #[serde(serialize_with = "option_string_as_reference::serialize")]
        r1: Option<String>,
        #[serde(serialize_with = "option_string_as_reference::serialize")]
        r2: Option<String>,
        s1: Option<String>,
        s2: Option<String>,
    }

    let o = S {
        r1: Some("projects/p/databases/d/documents/c/1".to_string()),
        r2: None,
        s1: Some("s1".to_string()),
        s2: None,
    };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut fields = std::collections::HashMap::new();
                fields.insert(
                    "r1".to_string(),
                    Value {
                        value_type: Some(ValueType::ReferenceValue(
                            "projects/p/databases/d/documents/c/1".to_string(),
                        )),
                    },
                );
                fields.insert(
                    "r2".to_string(),
                    Value {
                        value_type: Some(ValueType::NullValue(0)),
                    },
                );
                fields.insert(
                    "s1".to_string(),
                    Value {
                        value_type: Some(ValueType::StringValue("s1".to_string())),
                    },
                );
                fields.insert(
                    "s2".to_string(),
                    Value {
                        value_type: Some(ValueType::NullValue(0)),
                    },
                );
                fields
            },
        })),
    };
    let s = to_value(&o)?;
    assert_eq!(s, v);
    Ok(())
}
