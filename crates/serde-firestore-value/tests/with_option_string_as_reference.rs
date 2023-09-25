use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, MapValue, Value};
use serde_firestore_value::{from_value, to_value, with::option_string_as_reference};

#[test]
fn test_deserialize_with() -> anyhow::Result<()> {
    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct S {
        #[serde(deserialize_with = "option_string_as_reference::deserialize")]
        r1: Option<String>,
        #[serde(deserialize_with = "option_string_as_reference::deserialize")]
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
                let mut fields = HashMap::new();
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
    let d = from_value::<'_, S>(&v)?;
    assert_eq!(d, o);
    Ok(())
}

#[test]
fn test_serialize_with() -> anyhow::Result<()> {
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
                let mut fields = HashMap::new();
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

#[test]
fn test_with() -> anyhow::Result<()> {
    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S {
        #[serde(with = "option_string_as_reference")]
        r1: Option<String>,
        #[serde(with = "option_string_as_reference")]
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
                let mut fields = HashMap::new();
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
    let d = from_value::<'_, S>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
