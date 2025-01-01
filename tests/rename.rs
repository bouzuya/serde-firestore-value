#![allow(missing_docs)]

// BREAKING CHANGE 0.9.0: Change to stop using google_api_proto
//
// use std::collections::BTreeMap;

// use google_api_proto::google::firestore::v1::{value::ValueType, MapValue, Value};
// use serde_firestore_value::{from_value, to_value};

// #[test]
// fn test_rename() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//     struct S {
//         #[serde(rename = "x")]
//         a: bool,
//         #[serde(rename = "y")]
//         b: i64,
//         #[serde(rename = "z")]
//         c: String,
//     }

//     let cases = vec![(
//         S {
//             a: true,
//             b: 1_i64,
//             c: "s".to_string(),
//         },
//         Value {
//             value_type: Some(ValueType::MapValue(MapValue {
//                 fields: {
//                     let mut map = BTreeMap::new();
//                     map.insert(
//                         "x".to_string(),
//                         Value {
//                             value_type: Some(ValueType::BooleanValue(true)),
//                         },
//                     );
//                     map.insert(
//                         "y".to_string(),
//                         Value {
//                             value_type: Some(ValueType::IntegerValue(1_i64)),
//                         },
//                     );
//                     map.insert(
//                         "z".to_string(),
//                         Value {
//                             value_type: Some(ValueType::StringValue("s".to_string())),
//                         },
//                     );
//                     map
//                 },
//             })),
//         },
//     )];

//     for (o, v) in cases {
//         let s = to_value(&o)?;
//         let d = from_value::<'_, S>(&s)?;
//         assert_eq!(o, d);
//         assert_eq!(s, v);
//     }

//     Ok(())
// }

// #[test]
// fn test_rename_all_camel_case() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//     #[serde(rename_all = "camelCase")]
//     struct Person {
//         first_name: String,
//         last_name: String,
//     }

//     let cases = vec![(
//         Person {
//             first_name: "Graydon".to_string(),
//             last_name: "Hoare".to_string(),
//         },
//         Value {
//             value_type: Some(ValueType::MapValue(MapValue {
//                 fields: {
//                     let mut map = BTreeMap::new();
//                     map.insert(
//                         "firstName".to_string(),
//                         Value {
//                             value_type: Some(ValueType::StringValue("Graydon".to_string())),
//                         },
//                     );
//                     map.insert(
//                         "lastName".to_string(),
//                         Value {
//                             value_type: Some(ValueType::StringValue("Hoare".to_string())),
//                         },
//                     );
//                     map
//                 },
//             })),
//         },
//     )];

//     for (o, v) in cases {
//         let s = to_value(&o)?;
//         let d = from_value::<'_, Person>(&s)?;
//         assert_eq!(o, d);
//         assert_eq!(s, v);
//     }

//     Ok(())
// }
