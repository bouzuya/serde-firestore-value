use serde_firestore_value::from_value;

#[test]
fn test_pipeline() -> anyhow::Result<()> {
    use serde_firestore_value::{Pipeline, Stage, from_value, google, to_value};
    #[cfg(feature = "btree-map")]
    use std::collections::BTreeMap as Map;
    #[cfg(feature = "hash-map")]
    use std::collections::HashMap as Map;

    let o = Pipeline {
        stages: vec![
            Stage {
                name: "filter".to_owned(),
                args: vec![google::firestore::v1::Value {
                    value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                        "active = true".to_owned(),
                    )),
                }],
                options: Map::new(),
            },
            Stage {
                name: "sort".to_owned(),
                args: vec![google::firestore::v1::Value {
                    value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                        "created_at".to_owned(),
                    )),
                }],
                options: Map::new(),
            },
        ],
    };
    let v = google::firestore::v1::Value {
        value_type: Some(google::firestore::v1::value::ValueType::PipelineValue(
            google::firestore::v1::Pipeline {
                stages: vec![
                    google::firestore::v1::pipeline::Stage {
                        name: "filter".to_owned(),
                        args: vec![google::firestore::v1::Value {
                            value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                                "active = true".to_owned(),
                            )),
                        }],
                        options: Map::new(),
                    },
                    google::firestore::v1::pipeline::Stage {
                        name: "sort".to_owned(),
                        args: vec![google::firestore::v1::Value {
                            value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                                "created_at".to_owned(),
                            )),
                        }],
                        options: Map::new(),
                    },
                ],
            },
        )),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, Pipeline>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}

#[test]
fn test_pipeline_with_options() -> anyhow::Result<()> {
    use serde_firestore_value::{Pipeline, Stage, google, to_value};
    #[cfg(feature = "btree-map")]
    use std::collections::BTreeMap as Map;
    #[cfg(feature = "hash-map")]
    use std::collections::HashMap as Map;

    let o = Pipeline {
        stages: vec![Stage {
            name: "aggregate".to_owned(),
            args: vec![google::firestore::v1::Value {
                value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                    "count".to_owned(),
                )),
            }],
            options: {
                let mut opts = Map::new();
                opts.insert(
                    "alias".to_owned(),
                    google::firestore::v1::Value {
                        value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                            "total".to_owned(),
                        )),
                    },
                );
                opts
            },
        }],
    };
    let v = google::firestore::v1::Value {
        value_type: Some(google::firestore::v1::value::ValueType::PipelineValue(
            google::firestore::v1::Pipeline {
                stages: vec![google::firestore::v1::pipeline::Stage {
                    name: "aggregate".to_owned(),
                    args: vec![google::firestore::v1::Value {
                        value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                            "count".to_owned(),
                        )),
                    }],
                    options: {
                        let mut opts = Map::new();
                        opts.insert(
                            "alias".to_owned(),
                            google::firestore::v1::Value {
                                value_type: Some(
                                    google::firestore::v1::value::ValueType::StringValue(
                                        "total".to_owned(),
                                    ),
                                ),
                            },
                        );
                        opts
                    },
                }],
            },
        )),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, Pipeline>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}

#[test]
fn test_pipeline_with_nested_values() -> anyhow::Result<()> {
    use serde_firestore_value::{Pipeline, Stage, google, to_value};
    #[cfg(feature = "btree-map")]
    use std::collections::BTreeMap as Map;
    #[cfg(feature = "hash-map")]
    use std::collections::HashMap as Map;

    let o = Pipeline {
        stages: vec![Stage {
            name: "map".to_owned(),
            args: vec![google::firestore::v1::Value {
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
            }],
            options: Map::new(),
        }],
    };
    let v = google::firestore::v1::Value {
        value_type: Some(google::firestore::v1::value::ValueType::PipelineValue(
            google::firestore::v1::Pipeline {
                stages: vec![google::firestore::v1::pipeline::Stage {
                    name: "map".to_owned(),
                    args: vec![google::firestore::v1::Value {
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
                    }],
                    options: Map::new(),
                }],
            },
        )),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, Pipeline>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}

#[test]
fn test_pipeline_deserialize() -> anyhow::Result<()> {
    use serde_firestore_value::{Pipeline, Stage, google};
    #[cfg(feature = "btree-map")]
    use std::collections::BTreeMap as Map;
    #[cfg(feature = "hash-map")]
    use std::collections::HashMap as Map;

    let v = google::firestore::v1::Value {
        value_type: Some(google::firestore::v1::value::ValueType::PipelineValue(
            google::firestore::v1::Pipeline {
                stages: vec![google::firestore::v1::pipeline::Stage {
                    name: "select".to_owned(),
                    args: vec![
                        google::firestore::v1::Value {
                            value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                                "name".to_owned(),
                            )),
                        },
                        google::firestore::v1::Value {
                            value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                                "email".to_owned(),
                            )),
                        },
                    ],
                    options: Map::new(),
                }],
            },
        )),
    };

    let d = from_value::<'_, Pipeline>(&v)?;

    assert_eq!(
        d,
        Pipeline {
            stages: vec![Stage {
                name: "select".to_owned(),
                args: vec![
                    google::firestore::v1::Value {
                        value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                            "name".to_owned()
                        )),
                    },
                    google::firestore::v1::Value {
                        value_type: Some(google::firestore::v1::value::ValueType::StringValue(
                            "email".to_owned()
                        )),
                    },
                ],
                options: Map::new(),
            }],
        }
    );

    Ok(())
}
