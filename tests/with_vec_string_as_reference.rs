// BREAKING CHANGE 0.9.0: Change to stop using google_api_proto
//
// use std::collections::BTreeMap;

// use google_api_proto::google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};
// use serde_firestore_value::{from_value, to_value, with::vec_string_as_reference};

// #[test]
// fn test_deserialize_with() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
//     struct S {
//         #[serde(deserialize_with = "vec_string_as_reference::deserialize")]
//         r1: Vec<String>,
//         #[serde(deserialize_with = "vec_string_as_reference::deserialize")]
//         r2: Vec<String>,
//         s1: Vec<String>,
//         s2: Vec<String>,
//     }

//     let o = S {
//         r1: vec!["projects/p/databases/d/documents/c/1".to_string()],
//         r2: vec![],
//         s1: vec!["s1".to_string()],
//         s2: vec![],
//     };
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut fields = BTreeMap::new();
//                 fields.insert(
//                     "r1".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue {
//                             values: vec![Value {
//                                 value_type: Some(ValueType::ReferenceValue(
//                                     "projects/p/databases/d/documents/c/1".to_string(),
//                                 )),
//                             }],
//                         })),
//                     },
//                 );
//                 fields.insert(
//                     "r2".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![] })),
//                     },
//                 );
//                 fields.insert(
//                     "s1".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue {
//                             values: vec![Value {
//                                 value_type: Some(ValueType::StringValue("s1".to_string())),
//                             }],
//                         })),
//                     },
//                 );
//                 fields.insert(
//                     "s2".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![] })),
//                     },
//                 );
//                 fields
//             },
//         })),
//     };
//     let d = from_value::<'_, S>(&v)?;
//     assert_eq!(d, o);
//     Ok(())
// }

// #[test]
// fn test_serialize_with() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Serialize)]
//     struct S {
//         #[serde(serialize_with = "vec_string_as_reference::serialize")]
//         r1: Vec<String>,
//         #[serde(serialize_with = "vec_string_as_reference::serialize")]
//         r2: Vec<String>,
//         s1: Vec<String>,
//         s2: Vec<String>,
//     }

//     let o = S {
//         r1: vec!["projects/p/databases/d/documents/c/1".to_string()],
//         r2: vec![],
//         s1: vec!["s1".to_string()],
//         s2: vec![],
//     };
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut fields = BTreeMap::new();
//                 fields.insert(
//                     "r1".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue {
//                             values: vec![Value {
//                                 value_type: Some(ValueType::ReferenceValue(
//                                     "projects/p/databases/d/documents/c/1".to_string(),
//                                 )),
//                             }],
//                         })),
//                     },
//                 );
//                 fields.insert(
//                     "r2".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![] })),
//                     },
//                 );
//                 fields.insert(
//                     "s1".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue {
//                             values: vec![Value {
//                                 value_type: Some(ValueType::StringValue("s1".to_string())),
//                             }],
//                         })),
//                     },
//                 );
//                 fields.insert(
//                     "s2".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![] })),
//                     },
//                 );
//                 fields
//             },
//         })),
//     };
//     let s = to_value(&o)?;
//     assert_eq!(s, v);
//     Ok(())
// }

// #[test]
// fn test_with() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//     struct S {
//         #[serde(with = "vec_string_as_reference")]
//         r1: Vec<String>,
//         #[serde(with = "vec_string_as_reference")]
//         r2: Vec<String>,
//         s1: Vec<String>,
//         s2: Vec<String>,
//     }

//     let o = S {
//         r1: vec!["projects/p/databases/d/documents/c/1".to_string()],
//         r2: vec![],
//         s1: vec!["s1".to_string()],
//         s2: vec![],
//     };
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut fields = BTreeMap::new();
//                 fields.insert(
//                     "r1".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue {
//                             values: vec![Value {
//                                 value_type: Some(ValueType::ReferenceValue(
//                                     "projects/p/databases/d/documents/c/1".to_string(),
//                                 )),
//                             }],
//                         })),
//                     },
//                 );
//                 fields.insert(
//                     "r2".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![] })),
//                     },
//                 );
//                 fields.insert(
//                     "s1".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue {
//                             values: vec![Value {
//                                 value_type: Some(ValueType::StringValue("s1".to_string())),
//                             }],
//                         })),
//                     },
//                 );
//                 fields.insert(
//                     "s2".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![] })),
//                     },
//                 );
//                 fields
//             },
//         })),
//     };
//     let s = to_value(&o)?;
//     let d = from_value::<'_, S>(&s)?;
//     assert_eq!(s, v);
//     assert_eq!(d, o);
//     Ok(())
// }
