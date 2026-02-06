#[test]
fn test_function() -> anyhow::Result<()> {
    use serde_firestore_value::{Function, google, to_value};
    use std::collections::HashMap;

    let o = Function {
        name: "add".to_string(),
        args: vec![
            google::firestore::v1::Value {
                value_type: Some(google::firestore::v1::value::ValueType::IntegerValue(1)),
            },
            google::firestore::v1::Value {
                value_type: Some(google::firestore::v1::value::ValueType::IntegerValue(2)),
            },
        ],
        options: HashMap::new(),
    };
    let v = google::firestore::v1::Value {
        value_type: Some(google::firestore::v1::value::ValueType::FunctionValue(
            google::firestore::v1::Function {
                name: "add".to_string(),
                args: vec![
                    google::firestore::v1::Value {
                        value_type: Some(google::firestore::v1::value::ValueType::IntegerValue(1)),
                    },
                    google::firestore::v1::Value {
                        value_type: Some(google::firestore::v1::value::ValueType::IntegerValue(2)),
                    },
                ],
                options: HashMap::new(),
            },
        )),
    };
    let s = to_value(&o)?;
    assert_eq!(s, v);
    Ok(())
}

#[test]
fn test_function_with_options() -> anyhow::Result<()> {
    use serde_firestore_value::{Function, google, to_value};
    use std::collections::HashMap;

    let o = Function {
        name: "custom_func".to_string(),
        args: vec![google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                "arg1".to_string(),
            )),
        }],
        options: {
            let mut opts = HashMap::new();
            opts.insert(
                "timeout".to_string(),
                google::firestore::v1::Value {
                    value_type: Some(google::firestore::v1::value::ValueType::IntegerValue(30)),
                },
            );
            opts
        },
    };
    let v = google::firestore::v1::Value {
        value_type: Some(google::firestore::v1::value::ValueType::FunctionValue(
            google::firestore::v1::Function {
                name: "custom_func".to_string(),
                args: vec![google::firestore::v1::Value {
                    value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                        "arg1".to_string(),
                    )),
                }],
                options: {
                    let mut opts = HashMap::new();
                    opts.insert(
                        "timeout".to_string(),
                        google::firestore::v1::Value {
                            value_type: Some(
                                google::firestore::v1::value::ValueType::IntegerValue(30),
                            ),
                        },
                    );
                    opts
                },
            },
        )),
    };
    let s = to_value(&o)?;
    assert_eq!(s, v);
    Ok(())
}

#[test]
fn test_function_with_nested_values() -> anyhow::Result<()> {
    use serde_firestore_value::{Function, google, to_value};
    use std::collections::HashMap;

    let o = Function {
        name: "process".to_string(),
        args: vec![
            google::firestore::v1::Value {
                value_type: Some(google::firestore::v1::value::ValueType::ArrayValue(
                    google::firestore::v1::ArrayValue {
                        values: vec![
                            google::firestore::v1::Value {
                                value_type: Some(
                                    google::firestore::v1::value::ValueType::IntegerValue(1),
                                ),
                            },
                            google::firestore::v1::Value {
                                value_type: Some(
                                    google::firestore::v1::value::ValueType::IntegerValue(2),
                                ),
                            },
                        ],
                    },
                )),
            },
            google::firestore::v1::Value {
                value_type: Some(google::firestore::v1::value::ValueType::BooleanValue(true)),
            },
        ],
        options: HashMap::new(),
    };
    let v = google::firestore::v1::Value {
        value_type: Some(google::firestore::v1::value::ValueType::FunctionValue(
            google::firestore::v1::Function {
                name: "process".to_string(),
                args: vec![
                    google::firestore::v1::Value {
                        value_type: Some(google::firestore::v1::value::ValueType::ArrayValue(
                            google::firestore::v1::ArrayValue {
                                values: vec![
                                    google::firestore::v1::Value {
                                        value_type: Some(
                                            google::firestore::v1::value::ValueType::IntegerValue(
                                                1,
                                            ),
                                        ),
                                    },
                                    google::firestore::v1::Value {
                                        value_type: Some(
                                            google::firestore::v1::value::ValueType::IntegerValue(
                                                2,
                                            ),
                                        ),
                                    },
                                ],
                            },
                        )),
                    },
                    google::firestore::v1::Value {
                        value_type: Some(google::firestore::v1::value::ValueType::BooleanValue(
                            true,
                        )),
                    },
                ],
                options: HashMap::new(),
            },
        )),
    };
    let s = to_value(&o)?;
    assert_eq!(s, v);
    Ok(())
}
