#[test]
fn test_variable_reference() -> anyhow::Result<()> {
    use serde_firestore_value::{
        VariableReference, from_value,
        google::firestore::v1::{Value, value::ValueType},
        to_value,
    };

    let inner = "variable_name";
    let o = VariableReference(inner.to_string());
    let v = Value {
        value_type: Some(ValueType::VariableReferenceValue(inner.to_string())),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, VariableReference>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let json = serde_json::to_string(&o)?;
    assert_eq!(json, r#""variable_name""#);
    Ok(())
}

#[test]
fn test_variable_reference_value_in_struct() -> anyhow::Result<()> {
    use serde_firestore_value::{
        VariableReference, from_value,
        google::firestore::v1::{MapValue, Value, value::ValueType},
        to_value,
    };
    #[cfg(feature = "btree-map")]
    use std::collections::BTreeMap as Map;
    #[cfg(feature = "hash-map")]
    use std::collections::HashMap as Map;

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S {
        variable_ref: VariableReference,
    }

    let o = S {
        variable_ref: VariableReference("some_variable_nested".to_string()),
    };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut fields = Map::new();
                fields.insert(
                    "variable_ref".to_string(),
                    Value {
                        value_type: Some(ValueType::VariableReferenceValue(
                            "some_variable_nested".to_string(),
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
