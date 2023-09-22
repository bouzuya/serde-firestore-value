use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};
use serde_firestore_value::{from_value, to_value};

#[test]
fn test_struct() -> anyhow::Result<()> {
    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Test {
        int: u32,
        seq: Vec<String>,
    }

    let cases = vec![(
        Test {
            int: 1,
            seq: vec!["a".to_string(), "b".to_string()],
        },
        Value {
            value_type: Some(ValueType::MapValue(MapValue {
                fields: {
                    let mut map = HashMap::new();
                    map.insert(
                        "int".to_string(),
                        Value {
                            value_type: Some(ValueType::IntegerValue(1)),
                        },
                    );
                    map.insert(
                        "seq".to_string(),
                        Value {
                            value_type: Some(ValueType::ArrayValue(ArrayValue {
                                values: vec![
                                    Value {
                                        value_type: Some(ValueType::StringValue("a".to_string())),
                                    },
                                    Value {
                                        value_type: Some(ValueType::StringValue("b".to_string())),
                                    },
                                ],
                            })),
                        },
                    );
                    map
                },
            })),
        },
    )];

    for (o, v) in cases {
        let s = to_value(&o)?;
        let d = from_value::<'_, Test>(&s)?;
        assert_eq!(o, d);
        assert_eq!(s, v);
    }

    Ok(())
}

#[test]
fn test_enum() -> anyhow::Result<()> {
    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    let cases = vec![
        (
            E::Unit,
            Value {
                value_type: Some(ValueType::StringValue("Unit".to_string())),
            },
        ),
        (
            E::Newtype(1),
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: {
                        let mut map = HashMap::new();
                        map.insert(
                            "Newtype".to_string(),
                            Value {
                                value_type: Some(ValueType::IntegerValue(1)),
                            },
                        );
                        map
                    },
                })),
            },
        ),
        (
            E::Tuple(1, 2),
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: {
                        let mut map = HashMap::new();
                        map.insert(
                            "Tuple".to_string(),
                            Value {
                                value_type: Some(ValueType::ArrayValue(ArrayValue {
                                    values: vec![
                                        Value {
                                            value_type: Some(ValueType::IntegerValue(1)),
                                        },
                                        Value {
                                            value_type: Some(ValueType::IntegerValue(2)),
                                        },
                                    ],
                                })),
                            },
                        );
                        map
                    },
                })),
            },
        ),
        (
            E::Struct { a: 1 },
            Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: {
                        let mut map = HashMap::new();
                        map.insert(
                            "Struct".to_string(),
                            Value {
                                value_type: Some(ValueType::MapValue(MapValue {
                                    fields: {
                                        let mut map = HashMap::new();
                                        map.insert(
                                            "a".to_string(),
                                            Value {
                                                value_type: Some(ValueType::IntegerValue(1)),
                                            },
                                        );
                                        map
                                    },
                                })),
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
        let d = from_value::<'_, E>(&s)?;
        assert_eq!(o, d);
        assert_eq!(s, v);
    }

    Ok(())
}
