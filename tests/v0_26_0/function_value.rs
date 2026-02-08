use serde_firestore_value::from_value;

#[test]
fn test_function() -> anyhow::Result<()> {
    use serde_firestore_value::{Function, from_value, google, to_value};
    use std::collections::HashMap;

    let o = Function {
        name: "add".to_owned(),
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
                name: "add".to_owned(),
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
    let d = from_value::<'_, Function>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}

#[test]
fn test_function_with_options() -> anyhow::Result<()> {
    use serde_firestore_value::{Function, google, to_value};
    use std::collections::HashMap;

    let o = Function {
        name: "custom_func".to_owned(),
        args: vec![google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                "arg1".to_owned(),
            )),
        }],
        options: {
            let mut opts = HashMap::new();
            opts.insert(
                "timeout".to_owned(),
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
                name: "custom_func".to_owned(),
                args: vec![google::firestore::v1::Value {
                    value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                        "arg1".to_owned(),
                    )),
                }],
                options: {
                    let mut opts = HashMap::new();
                    opts.insert(
                        "timeout".to_owned(),
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
    let d = from_value::<'_, Function>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}

#[test]
fn test_function_with_nested_values() -> anyhow::Result<()> {
    use serde_firestore_value::{Function, google, to_value};
    use std::collections::HashMap;

    let o = Function {
        name: "process".to_owned(),
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
                name: "process".to_owned(),
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
    let d = from_value::<'_, Function>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}

#[test]
fn test_function_deserialize() -> anyhow::Result<()> {
    use serde_firestore_value::{Function, from_value, google, to_value};
    use std::collections::HashMap;

    // Create args with all ValueType variants (except PipelineValue which is not supported)
    let args = vec![
        // 1. NullValue
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::NullValue(0)),
        },
        // 2. BooleanValue
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::BooleanValue(true)),
        },
        // 3. IntegerValue
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::IntegerValue(42)),
        },
        // 4. DoubleValue
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::DoubleValue(3.14)),
        },
        // 5. TimestampValue
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::TimestampValue(
                prost_types::Timestamp {
                    seconds: 1234567890,
                    nanos: 123456789,
                },
            )),
        },
        // 6. StringValue
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                "hello".to_owned(),
            )),
        },
        // 7. BytesValue
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::BytesValue(vec![
                0x01, 0x02, 0x03,
            ])),
        },
        // 8. ReferenceValue
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::ReferenceValue(
                "projects/p/databases/d/documents/c/doc".to_owned(),
            )),
        },
        // 9. GeoPointValue
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::GeoPointValue(
                google::r#type::LatLng {
                    latitude: 35.6762,
                    longitude: 139.6503,
                },
            )),
        },
        // 10. ArrayValue
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
        // 11. MapValue
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::MapValue(
                google::firestore::v1::MapValue {
                    fields: {
                        let mut fields = HashMap::new();
                        fields.insert(
                            "key".to_owned(),
                            google::firestore::v1::Value {
                                value_type: Some(
                                    google::firestore::v1::value::ValueType::StringValue(
                                        "value".to_owned(),
                                    ),
                                ),
                            },
                        );
                        fields
                    },
                },
            )),
        },
        // 11 (2) . MapValue (TimestampValue like)
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::MapValue(
                google::firestore::v1::MapValue {
                    fields: {
                        let mut fields = HashMap::new();
                        fields.insert(
                            "seconds".to_owned(),
                            google::firestore::v1::Value {
                                value_type: Some(
                                    google::firestore::v1::value::ValueType::IntegerValue(123),
                                ),
                            },
                        );
                        fields.insert(
                            "nanos".to_owned(),
                            google::firestore::v1::Value {
                                value_type: Some(
                                    google::firestore::v1::value::ValueType::IntegerValue(456),
                                ),
                            },
                        );
                        fields
                    },
                },
            )),
        },
        // 11 (3) . MapValue (GeoPointValue like)
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::MapValue(
                google::firestore::v1::MapValue {
                    fields: {
                        let mut fields = HashMap::new();
                        fields.insert(
                            "latitude".to_owned(),
                            google::firestore::v1::Value {
                                value_type: Some(
                                    google::firestore::v1::value::ValueType::DoubleValue(35.6762),
                                ),
                            },
                        );
                        fields.insert(
                            "longitude".to_owned(),
                            google::firestore::v1::Value {
                                value_type: Some(
                                    google::firestore::v1::value::ValueType::DoubleValue(139.6503),
                                ),
                            },
                        );
                        fields
                    },
                },
            )),
        },
        // 11 (3) . MapValue (FunctionValue like)
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::MapValue(
                google::firestore::v1::MapValue {
                    fields: {
                        let mut fields = HashMap::new();
                        fields.insert(
                            "name".to_owned(),
                            google::firestore::v1::Value {
                                value_type: Some(
                                    google::firestore::v1::value::ValueType::StringValue(
                                        "func_name".to_owned(),
                                    ),
                                ),
                            },
                        );
                        fields.insert(
                            "args".to_owned(),
                            google::firestore::v1::Value {
                                value_type: Some(google::firestore::v1::value::ValueType::ArrayValue(
                                    google::firestore::v1::ArrayValue {
                                        values: vec![
                                            google::firestore::v1::Value {
                                                value_type: Some(
                                                    google::firestore::v1::value::ValueType::IntegerValue(10),
                                                ),
                                            },
                                            google::firestore::v1::Value {
                                                value_type: Some(
                                                    google::firestore::v1::value::ValueType::IntegerValue(20),
                                                ),
                                            },
                                        ],
                                    },
                                )),
                            },
                        );
                        fields.insert(
                            "options".to_owned(),
                            google::firestore::v1::Value {
                                value_type: Some(google::firestore::v1::value::ValueType::MapValue(
                                    google::firestore::v1::MapValue {
                                        fields: {
                                            let mut opts = HashMap::new();
                                            opts.insert(
                                                "opt_key".to_owned(),
                                                google::firestore::v1::Value {
                                                    value_type: Some(
                                                        google::firestore::v1::value::ValueType::StringValue(
                                                            "opt_value".to_owned(),
                                                        ),
                                                    ),
                                                },
                                            );
                                            opts
                                        },
                                    },
                                )),
                            },
                        );
                        fields
                    },
                },
            )),
        },
        // 11 (4) . MapValue (PipelineValue like)
        // FIXME
        // 12. FieldReferenceValue
        google::firestore::v1::Value {
            value_type: Some(
                google::firestore::v1::value::ValueType::FieldReferenceValue(
                    "field_name".to_owned(),
                ),
            ),
        },
        // 13. FunctionValue (nested)
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::FunctionValue(
                google::firestore::v1::Function {
                    name: "nested_func".to_owned(),
                    args: vec![google::firestore::v1::Value {
                        value_type: Some(google::firestore::v1::value::ValueType::IntegerValue(99)),
                    }],
                    options: HashMap::new(),
                },
            )),
        },
    ];

    let o = Function {
        name: "test_all_types".to_owned(),
        args,
        options: HashMap::new(),
    };

    let s = to_value(&o)?;
    let d = from_value::<'_, Function>(&s)?;
    assert_eq!(d, o);
    Ok(())
}
