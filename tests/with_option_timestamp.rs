#![allow(missing_docs)]

// BREAKING CHANGE 0.9.0: Change to stop using google_api_proto
//
// use std::collections::BTreeMap;

// use google_api_proto::google::firestore::v1::{value::ValueType, MapValue, Value};
// use serde_firestore_value::{from_value, to_value, with::option_timestamp};

// #[test]
// fn test_deserialize_with() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
//     struct S(
//         #[serde(deserialize_with = "option_timestamp::deserialize")] Option<prost_types::Timestamp>,
//     );

//     // some
//     {
//         let o = S(Some(prost_types::Timestamp {
//             seconds: 1_i64,
//             nanos: 2_i32,
//         }));
//         let v = Value {
//             value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//                 seconds: 1_i64,
//                 nanos: 2_i32,
//             })),
//         };
//         let d = from_value::<'_, S>(&v)?;
//         assert_eq!(d, o);
//     }

//     // none
//     {
//         let o = S(None);
//         let v = Value {
//             value_type: Some(ValueType::NullValue(0_i32)),
//         };
//         let d = from_value::<'_, S>(&v)?;
//         assert_eq!(d, o);
//     }
//     Ok(())
// }

// #[test]
// fn test_serialize_with() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Serialize)]
//     struct S(
//         #[serde(serialize_with = "option_timestamp::serialize")] Option<prost_types::Timestamp>,
//     );

//     // some
//     {
//         let o = S(Some(prost_types::Timestamp {
//             seconds: 1_i64,
//             nanos: 2_i32,
//         }));
//         let v = Value {
//             value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//                 seconds: 1_i64,
//                 nanos: 2_i32,
//             })),
//         };
//         let s = to_value(&o)?;
//         assert_eq!(s, v);
//     }

//     // none
//     {
//         let o = S(None);
//         let v = Value {
//             value_type: Some(ValueType::NullValue(0)),
//         };
//         let s = to_value(&o)?;
//         assert_eq!(s, v);
//     }
//     Ok(())
// }

// #[test]
// fn test_struct() -> anyhow::Result<()> {
//     #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//     struct S {
//         #[serde(with = "option_timestamp")]
//         a: Option<prost_types::Timestamp>,
//         #[serde(with = "option_timestamp")]
//         b: Option<prost_types::Timestamp>,
//     }
//     let o = S {
//         a: Some(prost_types::Timestamp {
//             seconds: 1_i64,
//             nanos: 2_i32,
//         }),
//         b: None,
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
//                 map.insert(
//                     "b".to_string(),
//                     Value {
//                         value_type: Some(ValueType::NullValue(0_i32)),
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
