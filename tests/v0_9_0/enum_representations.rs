// <https://serde.rs/enum-representations.html>

#[cfg(feature = "hash-map")]
#[test]
fn test_externally_tagged() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        value::ValueType, MapValue, Value,
    };
    use serde_firestore_value::{from_value, to_value};

    #[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
    enum E {
        A { f1: i64, f2: bool },
        B { f1: i64, f3: i64 },
    }

    let o = E::A { f1: 1, f2: true };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut fields = std::collections::HashMap::new();
                fields.insert(
                    "A".to_string(),
                    Value {
                        value_type: Some(ValueType::MapValue(MapValue {
                            fields: {
                                let mut fields = std::collections::HashMap::new();
                                fields.insert(
                                    "f1".to_string(),
                                    Value {
                                        value_type: Some(ValueType::IntegerValue(1)),
                                    },
                                );
                                fields.insert(
                                    "f2".to_string(),
                                    Value {
                                        value_type: Some(ValueType::BooleanValue(true)),
                                    },
                                );
                                fields
                            },
                        })),
                    },
                );
                fields
            },
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, E>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}

#[cfg(feature = "hash-map")]
#[test]
fn test_internally_tagged() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        value::ValueType, MapValue, Value,
    };
    use serde_firestore_value::{from_value, to_value};

    #[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "type")]
    enum E {
        A { f1: i64, f2: bool },
        B { f1: i64, f3: i64 },
    }

    let o = E::A { f1: 1, f2: true };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut fields = std::collections::HashMap::new();
                fields.insert(
                    "type".to_string(),
                    Value {
                        value_type: Some(ValueType::StringValue("A".to_string())),
                    },
                );
                fields.insert(
                    "f1".to_string(),
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                );
                fields.insert(
                    "f2".to_string(),
                    Value {
                        value_type: Some(ValueType::BooleanValue(true)),
                    },
                );
                fields
            },
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, E>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}

#[cfg(feature = "hash-map")]
#[test]
fn test_adjacently_tagged() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        value::ValueType, MapValue, Value,
    };
    use serde_firestore_value::{from_value, to_value};

    #[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "t", content = "c")]
    enum E {
        A { f1: i64, f2: bool },
        B { f1: i64, f3: i64 },
    }

    let o = E::A { f1: 1, f2: true };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut fields = std::collections::HashMap::new();
                fields.insert(
                    "t".to_string(),
                    Value {
                        value_type: Some(ValueType::StringValue("A".to_string())),
                    },
                );
                fields.insert(
                    "c".to_string(),
                    Value {
                        value_type: Some(ValueType::MapValue(MapValue {
                            fields: {
                                let mut fields = std::collections::HashMap::new();
                                fields.insert(
                                    "f1".to_string(),
                                    Value {
                                        value_type: Some(ValueType::IntegerValue(1)),
                                    },
                                );
                                fields.insert(
                                    "f2".to_string(),
                                    Value {
                                        value_type: Some(ValueType::BooleanValue(true)),
                                    },
                                );
                                fields
                            },
                        })),
                    },
                );
                fields
            },
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, E>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}

#[cfg(feature = "hash-map")]
#[test]
fn test_untagged() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        value::ValueType, MapValue, Value,
    };
    use serde_firestore_value::{from_value, to_value};

    #[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum E {
        A { f1: i64, f2: bool },
        B { f1: i64, f3: i64 },
    }

    let o = E::A { f1: 1, f2: true };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut fields = std::collections::HashMap::new();
                fields.insert(
                    "f1".to_string(),
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                );
                fields.insert(
                    "f2".to_string(),
                    Value {
                        value_type: Some(ValueType::BooleanValue(true)),
                    },
                );
                fields
            },
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, E>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
