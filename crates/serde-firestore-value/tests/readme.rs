use google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};

#[test]
fn test() -> anyhow::Result<()> {
    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct T {
        b: bool,
        i: i64,
        s: String,
        a: Vec<Option<i64>>,
        m: std::collections::HashMap<String, bool>,
    }

    let t = T {
        b: true,
        i: 1,
        s: "s".to_string(),
        a: vec![Some(1), Some(2), None],
        m: {
            let mut m = std::collections::HashMap::new();
            m.insert("a".to_string(), false);
            m.insert("b".to_string(), true);
            m
        },
    };
    let value = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut fields = std::collections::HashMap::new();
                fields.insert(
                    "b".to_string(),
                    Value {
                        value_type: Some(ValueType::BooleanValue(true)),
                    },
                );
                fields.insert(
                    "i".to_string(),
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                );
                fields.insert(
                    "s".to_string(),
                    Value {
                        value_type: Some(ValueType::StringValue("s".to_string())),
                    },
                );
                fields.insert(
                    "a".to_string(),
                    Value {
                        value_type: Some(ValueType::ArrayValue(ArrayValue {
                            values: vec![
                                Value {
                                    value_type: Some(ValueType::IntegerValue(1)),
                                },
                                Value {
                                    value_type: Some(ValueType::IntegerValue(2)),
                                },
                                Value {
                                    value_type: Some(ValueType::NullValue(0)),
                                },
                            ],
                        })),
                    },
                );
                fields.insert(
                    "m".to_string(),
                    Value {
                        value_type: Some(ValueType::MapValue(MapValue {
                            fields: {
                                let mut fields = std::collections::HashMap::new();
                                fields.insert(
                                    "a".to_string(),
                                    Value {
                                        value_type: Some(ValueType::BooleanValue(false)),
                                    },
                                );
                                fields.insert(
                                    "b".to_string(),
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

    let serialized = serde_firestore_value::to_value(&t)?;
    assert_eq!(serialized, value);

    let deserialized = serde_firestore_value::from_value::<T>(&serialized)?;
    assert_eq!(deserialized, t);

    Ok(())
}
