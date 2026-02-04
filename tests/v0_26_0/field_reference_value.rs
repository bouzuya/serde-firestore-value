#[test]
fn test_deserialize_field_reference_value_as_field_reference() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{Value, value::ValueType};
    use serde_firestore_value::{FieldReference, from_value};

    let v = Value {
        value_type: Some(ValueType::FieldReferenceValue("field_name".to_string())),
    };
    let d = from_value::<'_, FieldReference>(&v)?;
    assert_eq!(d, FieldReference("field_name".to_string()));
    Ok(())
}

#[test]
fn test_deserialize_field_reference_value_in_struct() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        MapValue, Value, value::ValueType,
    };
    use serde_firestore_value::{FieldReference, from_value};
    use std::collections::HashMap;

    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct S {
        field_ref: FieldReference,
    }

    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut fields = HashMap::new();
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
    let d = from_value::<'_, S>(&v)?;
    assert_eq!(
        d,
        S {
            field_ref: FieldReference("some_field.nested".to_string())
        }
    );
    Ok(())
}
