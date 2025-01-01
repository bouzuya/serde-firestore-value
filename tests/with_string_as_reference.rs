#![allow(missing_docs)]

// BREAKING CHANGE 0.9.0: Change to stop using google_api_proto
//
// use std::collections::BTreeMap;

// use google_api_proto::google::firestore::v1::{value::ValueType, MapValue, Value};
// use serde_firestore_value::{from_value, to_value, with::string_as_reference};

// #[test]
// fn test_deserialize_with() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
//     struct S {
//         #[serde(deserialize_with = "string_as_reference::deserialize")]
//         r: String,
//         s: String,
//     }

//     let o = S {
//         r: "projects/p/databases/d/documents/c/1".to_string(),
//         s: "s1".to_string(),
//     };
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut fields = BTreeMap::new();
//                 fields.insert(
//                     "r".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ReferenceValue(
//                             "projects/p/databases/d/documents/c/1".to_string(),
//                         )),
//                     },
//                 );
//                 fields.insert(
//                     "s".to_string(),
//                     Value {
//                         value_type: Some(ValueType::StringValue("s1".to_string())),
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
//         #[serde(serialize_with = "string_as_reference::serialize")]
//         r: String,
//         s: String,
//     }

//     let o = S {
//         r: "projects/p/databases/d/documents/c/1".to_string(),
//         s: "s1".to_string(),
//     };
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut fields = BTreeMap::new();
//                 fields.insert(
//                     "r".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ReferenceValue(
//                             "projects/p/databases/d/documents/c/1".to_string(),
//                         )),
//                     },
//                 );
//                 fields.insert(
//                     "s".to_string(),
//                     Value {
//                         value_type: Some(ValueType::StringValue("s1".to_string())),
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
//         #[serde(with = "string_as_reference")]
//         r: String,
//         s: String,
//     }

//     let o = S {
//         r: "projects/p/databases/d/documents/c/1".to_string(),
//         s: "s1".to_string(),
//     };
//     let v = Value {
//         value_type: Some(ValueType::MapValue(MapValue {
//             fields: {
//                 let mut fields = BTreeMap::new();
//                 fields.insert(
//                     "r".to_string(),
//                     Value {
//                         value_type: Some(ValueType::ReferenceValue(
//                             "projects/p/databases/d/documents/c/1".to_string(),
//                         )),
//                     },
//                 );
//                 fields.insert(
//                     "s".to_string(),
//                     Value {
//                         value_type: Some(ValueType::StringValue("s1".to_string())),
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
