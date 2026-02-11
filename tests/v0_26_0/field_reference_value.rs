#[test]
fn test_field_reference() -> anyhow::Result<()> {
    use serde_firestore_value::{
        FieldReference, from_value,
        google::firestore::v1::{Value, value::ValueType},
        to_value,
    };

    let inner = "field_name";
    let o = FieldReference(inner.to_string());
    let v = Value {
        value_type: Some(ValueType::FieldReferenceValue(inner.to_string())),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, FieldReference>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let json = serde_json::to_string(&o)?;
    assert_eq!(json, r#""field_name""#);
    Ok(())
}

#[test]
fn test_field_reference_value_in_struct() -> anyhow::Result<()> {
    use serde_firestore_value::{
        FieldReference, from_value,
        google::firestore::v1::{MapValue, Value, value::ValueType},
        to_value,
    };
    #[cfg(feature = "btree-map")]
    use std::collections::BTreeMap as Map;
    #[cfg(feature = "hash-map")]
    use std::collections::HashMap as Map;

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S {
        field_ref: FieldReference,
    }

    let o = S {
        field_ref: FieldReference("some_field.nested".to_string()),
    };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut fields = Map::new();
                fields.insert(
                    "field_ref".to_string(),
                    Value {
                        value_type: Some(ValueType::FieldReferenceValue(
                            "some_field.nested".to_string(),
                        )),
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
