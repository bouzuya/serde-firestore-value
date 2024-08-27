#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        value::ValueType, MapValue, Value,
    };
    use serde_firestore_value::{from_value, to_value, with::string_as_reference};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S {
        #[serde(with = "string_as_reference")]
        r: String,
        s: String,
    }

    let o = S {
        r: "projects/p/databases/d/documents/c/1".to_string(),
        s: "s1".to_string(),
    };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut fields = std::collections::HashMap::new();
                fields.insert(
                    "r".to_string(),
                    Value {
                        value_type: Some(ValueType::ReferenceValue(
                            "projects/p/databases/d/documents/c/1".to_string(),
                        )),
                    },
                );
                fields.insert(
                    "s".to_string(),
                    Value {
                        value_type: Some(ValueType::StringValue("s1".to_string())),
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
