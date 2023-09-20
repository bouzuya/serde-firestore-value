use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};
use serde_firestore_value::to_value;

#[test]
fn test_struct() -> anyhow::Result<()> {
    #[derive(serde::Serialize)]
    struct Test {
        int: u32,
        seq: Vec<&'static str>,
    }

    let test = Test {
        int: 1,
        seq: vec!["a", "b"],
    };
    assert_eq!(
        to_value(&test)?,
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
                }
            }))
        }
    );
    Ok(())
}

#[test]
fn test_enum() -> anyhow::Result<()> {
    #[derive(serde::Serialize)]
    enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    let u = E::Unit;
    assert_eq!(
        to_value(&u)?,
        Value {
            value_type: Some(ValueType::StringValue("Unit".to_string()))
        }
    );

    let n = E::Newtype(1);
    assert_eq!(
        to_value(&n)?,
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
                }
            }))
        }
    );

    let t = E::Tuple(1, 2);
    assert_eq!(
        to_value(&t)?,
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
                }
            }))
        }
    );

    let s = E::Struct { a: 1 };
    assert_eq!(
        to_value(&s)?,
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
                }
            }))
        }
    );
    Ok(())
}
