#![allow(missing_docs)]

// BREAKING CHANGE 0.9.0: Change to stop using google_api_proto
//
// use std::collections::BTreeMap;

// use google_api_proto::google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};
// use serde_firestore_value::{from_value, to_value, with::timestamp};

// #[test]
// fn test_deserialize_with() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
//     struct S(#[serde(deserialize_with = "timestamp::deserialize")] prost_types::Timestamp);

//     let o = S(prost_types::Timestamp {
//         seconds: 1_i64,
//         nanos: 2_i32,
//     });
//     let v = Value {
//         value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//             seconds: 1_i64,
//             nanos: 2_i32,
//         })),
//     };
//     let d = from_value::<'_, S>(&v)?;
//     assert_eq!(d, o);
//     Ok(())
// }

// #[test]
// fn test_serialize_with() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Serialize)]
//     struct S(#[serde(serialize_with = "timestamp::serialize")] prost_types::Timestamp);

//     let o = S(prost_types::Timestamp {
//         seconds: 1_i64,
//         nanos: 2_i32,
//     });
//     let v = Value {
//         value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//             seconds: 1_i64,
//             nanos: 2_i32,
//         })),
//     };
//     let s = to_value(&o)?;
//     assert_eq!(s, v);
//     Ok(())
// }

// // TODO: prost_types::Timestamp -> Value

// #[test]
// fn test_newtype_struct() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//     struct S(#[serde(with = "timestamp")] prost_types::Timestamp);

//     let o = S(prost_types::Timestamp {
//         seconds: 1_i64,
//         nanos: 2_i32,
//     });
//     let v = Value {
//         value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//             seconds: 1_i64,
//             nanos: 2_i32,
//         })),
//     };
//     let s = to_value(&o)?;
//     let d = from_value::<'_, S>(&s)?;
//     assert_eq!(s, v);
//     assert_eq!(d, o);
//     Ok(())
// }

// #[test]
// fn test_newtype_variant() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//     enum E {
//         A(#[serde(with = "timestamp")] prost_types::Timestamp),
//     }
//     let o = E::A(prost_types::Timestamp {
//         seconds: 1_i64,
//         nanos: 2_i32,
//     });
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut map = BTreeMap::new();
//                 map.insert(
//                     "A".to_string(),
//                     Value {
//                         value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//                             seconds: 1_i64,
//                             nanos: 2_i32,
//                         })),
//                     },
//                 );
//                 map
//             },
//         })),
//     };
//     let s = to_value(&o)?;
//     let d = from_value::<'_, E>(&s)?;
//     assert_eq!(s, v);
//     assert_eq!(d, o);
//     Ok(())
// }

// // TODO: seq (Vec<Timestamp> -> Value)
// // TODO: tuple ((prost_types::Timestamp,) -> Value)

// #[test]
// fn test_tuple_struct() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//     struct S(#[serde(with = "timestamp")] prost_types::Timestamp, bool);
//     let o = S(
//         prost_types::Timestamp {
//             seconds: 1_i64,
//             nanos: 2_i32,
//         },
//         true,
//     );
//     let v = Value {
//         value_type: Some(ValueType::ArrayValue(ArrayValue {
//             values: vec![
//                 Value {
//                     value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//                         seconds: 1_i64,
//                         nanos: 2_i32,
//                     })),
//                 },
//                 Value {
//                     value_type: Some(ValueType::BooleanValue(true)),
//                 },
//             ],
//         })),
//     };
//     let s = to_value(&o)?;
//     let d = from_value::<'_, S>(&s)?;
//     assert_eq!(s, v);
//     assert_eq!(d, o);
//     Ok(())
// }

// #[test]
// fn test_tuple_variant() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//     enum E {
//         A(#[serde(with = "timestamp")] prost_types::Timestamp, bool),
//     }
//     let o = E::A(
//         prost_types::Timestamp {
//             seconds: 1_i64,
//             nanos: 2_i32,
//         },
//         true,
//     );
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut map = BTreeMap::new();
//                 map.insert(
//                     "A".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue {
//                             values: vec![
//                                 Value {
//                                     value_type: Some(ValueType::TimestampValue(
//                                         prost_types::Timestamp {
//                                             seconds: 1_i64,
//                                             nanos: 2_i32,
//                                         },
//                                     )),
//                                 },
//                                 Value {
//                                     value_type: Some(ValueType::BooleanValue(true)),
//                                 },
//                             ],
//                         })),
//                     },
//                 );
//                 map
//             },
//         })),
//     };
//     let s = to_value(&o)?;
//     let d = from_value::<'_, E>(&s)?;
//     assert_eq!(s, v);
//     assert_eq!(d, o);
//     Ok(())
// }

// // TODO: map

// #[test]
// fn test_struct() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//     struct S {
//         #[serde(with = "timestamp")]
//         a: prost_types::Timestamp,
//     }
//     let o = S {
//         a: prost_types::Timestamp {
//             seconds: 1_i64,
//             nanos: 2_i32,
//         },
//     };
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut map = BTreeMap::new();
//                 map.insert(
//                     "a".to_string(),
//                     Value {
//                         value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//                             seconds: 1_i64,
//                             nanos: 2_i32,
//                         })),
//                     },
//                 );
//                 map
//             },
//         })),
//     };
//     let s = to_value(&o)?;
//     let d = from_value::<'_, S>(&s)?;
//     assert_eq!(s, v);
//     assert_eq!(d, o);
//     Ok(())
// }

// #[test]
// fn test_struct_variant() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//     enum E {
//         S {
//             #[serde(with = "timestamp")]
//             a: prost_types::Timestamp,
//         },
//     }
//     let o = E::S {
//         a: prost_types::Timestamp {
//             seconds: 1_i64,
//             nanos: 2_i32,
//         },
//     };
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut map = BTreeMap::new();
//                 map.insert(
//                     "S".to_string(),
//                     Value {
//                         value_type: Some(ValueType::MapValue(MapValue {
//                             fields: {
//                                 let mut map = BTreeMap::new();
//                                 map.insert(
//                                     "a".to_string(),
//                                     Value {
//                                         value_type: Some(ValueType::TimestampValue(
//                                             prost_types::Timestamp {
//                                                 seconds: 1_i64,
//                                                 nanos: 2_i32,
//                                             },
//                                         )),
//                                     },
//                                 );
//                                 map
//                             },
//                         })),
//                     },
//                 );
//                 map
//             },
//         })),
//     };
//     let s = to_value(&o)?;
//     let d = from_value::<'_, E>(&s)?;
//     assert_eq!(s, v);
//     assert_eq!(d, o);
//     Ok(())
// }
