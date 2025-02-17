#![allow(missing_docs)]

// BREAKING CHANGE 0.9.0: Change to stop using google_api_proto
//
// use std::collections::BTreeMap;

// use google_api_proto::google::firestore::v1::{value::ValueType, MapValue, Value};
// use serde_firestore_value::{from_value, to_value};

// #[test]
// fn test_externally_tagged() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
//     enum E {
//         A { f1: i64, f2: bool },
//         B { f1: i64, f3: i64 },
//     }

//     let o = E::A { f1: 1, f2: true };
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut fields = BTreeMap::new();
//                 fields.insert(
//                     "A".to_string(),
//                     Value {
//                         value_type: Some(ValueType::MapValue(MapValue {
//                             fields: {
//                                 let mut fields = BTreeMap::new();
//                                 fields.insert(
//                                     "f1".to_string(),
//                                     Value {
//                                         value_type: Some(ValueType::IntegerValue(1)),
//                                     },
//                                 );
//                                 fields.insert(
//                                     "f2".to_string(),
//                                     Value {
//                                         value_type: Some(ValueType::BooleanValue(true)),
//                                     },
//                                 );
//                                 fields
//                             },
//                         })),
//                     },
//                 );
//                 fields
//             },
//         })),
//     };
//     let s = to_value(&o)?;
//     let d = from_value::<'_, E>(&s)?;
//     assert_eq!(s, v);
//     assert_eq!(d, o);
//     Ok(())
// }

// #[test]
// fn test_internally_tagged() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
//     #[serde(tag = "type")]
//     enum E {
//         A { f1: i64, f2: bool },
//         B { f1: i64, f3: i64 },
//     }

//     let o = E::A { f1: 1, f2: true };
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut fields = BTreeMap::new();
//                 fields.insert(
//                     "type".to_string(),
//                     Value {
//                         value_type: Some(ValueType::StringValue("A".to_string())),
//                     },
//                 );
//                 fields.insert(
//                     "f1".to_string(),
//                     Value {
//                         value_type: Some(ValueType::IntegerValue(1)),
//                     },
//                 );
//                 fields.insert(
//                     "f2".to_string(),
//                     Value {
//                         value_type: Some(ValueType::BooleanValue(true)),
//                     },
//                 );
//                 fields
//             },
//         })),
//     };
//     let s = to_value(&o)?;
//     let d = from_value::<'_, E>(&s)?;
//     assert_eq!(s, v);
//     assert_eq!(d, o);
//     Ok(())
// }

// #[test]
// fn test_adjacently_tagged() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
//     #[serde(tag = "t", content = "c")]
//     enum E {
//         A { f1: i64, f2: bool },
//         B { f1: i64, f3: i64 },
//     }

//     let o = E::A { f1: 1, f2: true };
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut fields = BTreeMap::new();
//                 fields.insert(
//                     "t".to_string(),
//                     Value {
//                         value_type: Some(ValueType::StringValue("A".to_string())),
//                     },
//                 );
//                 fields.insert(
//                     "c".to_string(),
//                     Value {
//                         value_type: Some(ValueType::MapValue(MapValue {
//                             fields: {
//                                 let mut fields = BTreeMap::new();
//                                 fields.insert(
//                                     "f1".to_string(),
//                                     Value {
//                                         value_type: Some(ValueType::IntegerValue(1)),
//                                     },
//                                 );
//                                 fields.insert(
//                                     "f2".to_string(),
//                                     Value {
//                                         value_type: Some(ValueType::BooleanValue(true)),
//                                     },
//                                 );
//                                 fields
//                             },
//                         })),
//                     },
//                 );
//                 fields
//             },
//         })),
//     };
//     let s = to_value(&o)?;
//     let d = from_value::<'_, E>(&s)?;
//     assert_eq!(s, v);
//     assert_eq!(d, o);
//     Ok(())
// }

// #[test]
// fn test_untagged() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
//     #[serde(untagged)]
//     enum E {
//         A { f1: i64, f2: bool },
//         B { f1: i64, f3: i64 },
//     }

//     let o = E::A { f1: 1, f2: true };
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut fields = BTreeMap::new();
//                 fields.insert(
//                     "f1".to_string(),
//                     Value {
//                         value_type: Some(ValueType::IntegerValue(1)),
//                     },
//                 );
//                 fields.insert(
//                     "f2".to_string(),
//                     Value {
//                         value_type: Some(ValueType::BooleanValue(true)),
//                     },
//                 );
//                 fields
//             },
//         })),
//     };
//     let s = to_value(&o)?;
//     let d = from_value::<'_, E>(&s)?;
//     assert_eq!(s, v);
//     assert_eq!(d, o);
//     Ok(())
// }
