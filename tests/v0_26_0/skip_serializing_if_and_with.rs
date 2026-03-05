#[cfg(feature = "hash-map")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        MapValue, Value, value::ValueType,
    };
    use serde_firestore_value::{from_value, to_value, with::option_string_as_reference};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S1 {
        #[serde(
            skip_serializing_if = "Option::is_none",
            with = "option_string_as_reference"
        )]
        r: Option<String>,
    }

    let o = S1 {
        r: Some("projects/p/databases/d/documents/c/1".to_string()),
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
                fields
            },
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, S1>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let o = S1 { r: None };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: std::collections::HashMap::new(),
        })),
    };
    let s = to_value(&o)?;
    assert_eq!(s, v);
    assert_eq!(
        from_value::<'_, S1>(&s).unwrap_err().to_string(),
        "missing field `r`"
    );

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S2 {
        #[serde(
            // default
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_string_as_reference"
        )]
        r: Option<String>,
    }

    let o = S2 {
        r: Some("projects/p/databases/d/documents/c/1".to_string()),
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
                fields
            },
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, S2>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let o = S2 { r: None };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: std::collections::HashMap::new(),
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, S2>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o); // default allows deserialization to succeed with None
    Ok(())
}
