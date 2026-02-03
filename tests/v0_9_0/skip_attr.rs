#[cfg(feature = "hash-map")]
#[test]
fn test_skip() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        MapValue, Value, value::ValueType,
    };
    use serde_firestore_value::{from_value, to_value};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S {
        #[serde(skip)]
        a: bool,
        b: i64,
    }

    let o = S { a: true, b: 1_i64 };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    "b".to_string(),
                    Value {
                        value_type: Some(ValueType::IntegerValue(1_i64)),
                    },
                );
                map
            },
        })),
    };
    let s = to_value(&o)?;
    assert_eq!(s, v);
    assert_eq!(from_value::<'_, S>(&s)?, S { a: false, b: 1_i64 });

    Ok(())
}

#[cfg(feature = "hash-map")]
#[test]
fn test_skip_serializing_if_is_none() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        MapValue, Value, value::ValueType,
    };
    use serde_firestore_value::{from_value, to_value};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S1 {
        #[serde(skip_serializing_if = "Option::is_none")]
        a: Option<i64>,
        b: bool,
    }

    let cases = vec![
        (
            S1 { a: None, b: true },
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: {
                        let mut map = std::collections::HashMap::new();
                        // a is skipped
                        map.insert(
                            "b".to_string(),
                            Value {
                                value_type: Some(ValueType::BooleanValue(true)),
                            },
                        );
                        map
                    },
                })),
            },
        ),
        (
            S1 {
                a: Some(1_i64),
                b: true,
            },
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: {
                        let mut map = std::collections::HashMap::new();
                        map.insert(
                            "a".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(1_i64)),
                            },
                        );
                        map.insert(
                            "b".to_string(),
                            Value {
                                value_type: Some(ValueType::BooleanValue(true)),
                            },
                        );
                        map
                    },
                })),
            },
        ),
    ];

    for (o, v) in cases {
        let s = to_value(&o)?;
        let d = from_value::<'_, S1>(&s)?;
        assert_eq!(o, d);
        assert_eq!(s, v);
    }

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S2 {
        a: Option<i64>,
        b: bool,
    }
    let cases = vec![
        (
            S2 { a: None, b: true },
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: {
                        let mut map = std::collections::HashMap::new();
                        // a is not skipped
                        map.insert(
                            "a".to_string(),
                            Value {
                                value_type: Some(ValueType::NullValue(0_i32)),
                            },
                        );
                        map.insert(
                            "b".to_string(),
                            Value {
                                value_type: Some(ValueType::BooleanValue(true)),
                            },
                        );
                        map
                    },
                })),
            },
        ),
        (
            S2 {
                a: Some(1_i64),
                b: true,
            },
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: {
                        let mut map = std::collections::HashMap::new();
                        map.insert(
                            "a".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(1_i64)),
                            },
                        );
                        map.insert(
                            "b".to_string(),
                            Value {
                                value_type: Some(ValueType::BooleanValue(true)),
                            },
                        );
                        map
                    },
                })),
            },
        ),
    ];

    for (o, v) in cases {
        let s = to_value(&o)?;
        let d = from_value::<'_, S2>(&s)?;
        assert_eq!(o, d);
        assert_eq!(s, v);
    }

    Ok(())
}
