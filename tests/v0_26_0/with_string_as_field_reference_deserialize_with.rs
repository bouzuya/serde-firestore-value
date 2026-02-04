#[cfg(feature = "hash-map")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        MapValue, Value, value::ValueType,
    };
    use serde_firestore_value::{from_value, with::string_as_field_reference};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct S {
        #[serde(deserialize_with = "string_as_field_reference::deserialize")]
        r: String,
        s: String,
    }

    let o = S {
        r: "field_name".to_string(),
        s: "s1".to_string(),
    };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut fields = std::collections::HashMap::new();
                fields.insert(
                    "r".to_string(),
                    Value {
                        value_type: Some(ValueType::FieldReferenceValue("field_name".to_string())),
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
    let d = from_value::<'_, S>(&v)?;
    assert_eq!(d, o);
    Ok(())
}
