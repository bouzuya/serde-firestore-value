use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, MapValue, Value};
use serde_firestore_value::{to_value, with::string_as_reference};

#[test]
fn test_serialize_with() -> anyhow::Result<()> {
    #[derive(Debug, Eq, PartialEq, serde::Serialize)]
    struct S {
        #[serde(serialize_with = "string_as_reference::serialize")]
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
                let mut fields = HashMap::new();
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
    assert_eq!(s, v);
    Ok(())
}

// TODO: deserialize_with
