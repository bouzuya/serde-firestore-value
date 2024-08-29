// BREAKING CHANGE 0.9.0: Change to stop using google_api_proto
//
// #[cfg(feature = "chrono")]
// mod chrono_feature {
//     use google_api_proto::google::firestore::v1::{value::ValueType, Value};
//     use serde_firestore_value::{from_value, to_value, with::option_chrono_date_time_as_timestamp};

//     #[test]
//     fn test_deserialize_with_option_chrono_date_time_as_timestamp_deserialize() -> anyhow::Result<()>
//     {
//         #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
//         struct S(
//             #[serde(deserialize_with = "option_chrono_date_time_as_timestamp::deserialize")]
//             Option<chrono::DateTime<chrono::Utc>>,
//         );

//         let o = S(Some(
//             chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
//                 "1970-01-01T00:00:01.000000002Z",
//             )?
//             .with_timezone(&chrono::Utc),
//         ));
//         let v = Value {
//             value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//                 seconds: 1_i64,
//                 nanos: 2_i32,
//             })),
//         };
//         let d = from_value::<'_, S>(&v)?;
//         assert_eq!(d, o);

//         let o = S(None);
//         let v = Value {
//             value_type: Some(ValueType::NullValue(0)),
//         };
//         let d = from_value::<'_, S>(&v)?;
//         assert_eq!(d, o);
//         Ok(())
//     }

//     #[test]
//     fn test_serialize_with_option_chrono_date_time_as_timestamp_serialize() -> anyhow::Result<()> {
//         #[derive(Debug, Eq, PartialEq, serde::Serialize)]
//         struct S(
//             #[serde(serialize_with = "option_chrono_date_time_as_timestamp::serialize")]
//             Option<chrono::DateTime<chrono::Utc>>,
//         );

//         let o = S(Some(
//             chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
//                 "1970-01-01T00:00:01.000000002Z",
//             )?
//             .with_timezone(&chrono::Utc),
//         ));
//         let v = Value {
//             value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//                 seconds: 1_i64,
//                 nanos: 2_i32,
//             })),
//         };
//         let s = to_value(&o)?;
//         assert_eq!(s, v);

//         let o = S(None);
//         let v = Value {
//             value_type: Some(ValueType::NullValue(0)),
//         };
//         let s = to_value(&o)?;
//         assert_eq!(s, v);
//         Ok(())
//     }

//     #[test]
//     fn test_with_option_chrono_date_time_as_timestamp() -> anyhow::Result<()> {
//         #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//         struct S(
//             #[serde(with = "option_chrono_date_time_as_timestamp")]
//             Option<chrono::DateTime<chrono::Utc>>,
//         );

//         let o = S(Some(
//             chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
//                 "1970-01-01T00:00:01.000000002Z",
//             )?
//             .with_timezone(&chrono::Utc),
//         ));
//         let v = Value {
//             value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//                 seconds: 1_i64,
//                 nanos: 2_i32,
//             })),
//         };
//         let s = to_value(&o)?;
//         let d = from_value::<'_, S>(&s)?;
//         assert_eq!(s, v);
//         assert_eq!(d, o);

//         let o = S(None);
//         let v = Value {
//             value_type: Some(ValueType::NullValue(0)),
//         };
//         let s = to_value(&o)?;
//         let d = from_value::<'_, S>(&s)?;
//         assert_eq!(s, v);
//         assert_eq!(d, o);
//         Ok(())
//     }
// }

// #[cfg(feature = "time")]
// mod time_feature {
//     use google_api_proto::google::firestore::v1::{value::ValueType, Value};
//     use serde_firestore_value::{
//         from_value, to_value, with::option_time_offset_date_time_as_timestamp,
//     };

//     #[test]
//     fn test_deserialize_with() -> anyhow::Result<()> {
//         #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
//         struct S(
//             #[serde(deserialize_with = "option_time_offset_date_time_as_timestamp::deserialize")]
//             Option<time::OffsetDateTime>,
//         );

//         let o = S(Some(time::OffsetDateTime::from_unix_timestamp_nanos(
//             1_000_000_002_i128,
//         )?));
//         let v = Value {
//             value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//                 seconds: 1_i64,
//                 nanos: 2_i32,
//             })),
//         };
//         let d = from_value::<'_, S>(&v)?;
//         assert_eq!(d, o);

//         let o = S(None);
//         let v = Value {
//             value_type: Some(ValueType::NullValue(0)),
//         };
//         let d = from_value::<'_, S>(&v)?;
//         assert_eq!(d, o);
//         Ok(())
//     }

//     #[test]
//     fn test_serialize_with() -> anyhow::Result<()> {
//         #[derive(Debug, Eq, PartialEq, serde::Serialize)]
//         struct S(
//             #[serde(serialize_with = "option_time_offset_date_time_as_timestamp::serialize")]
//             Option<time::OffsetDateTime>,
//         );

//         let o = S(Some(time::OffsetDateTime::from_unix_timestamp_nanos(
//             1_000_000_002_i128,
//         )?));
//         let v = Value {
//             value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//                 seconds: 1_i64,
//                 nanos: 2_i32,
//             })),
//         };
//         let s = to_value(&o)?;
//         assert_eq!(s, v);

//         let o = S(None);
//         let v = Value {
//             value_type: Some(ValueType::NullValue(0)),
//         };
//         let s = to_value(&o)?;
//         assert_eq!(s, v);
//         Ok(())
//     }

//     #[test]
//     fn test_with() -> anyhow::Result<()> {
//         #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//         struct S(
//             #[serde(with = "option_time_offset_date_time_as_timestamp")]
//             Option<time::OffsetDateTime>,
//         );

//         let o = S(Some(time::OffsetDateTime::from_unix_timestamp_nanos(
//             1_000_000_002_i128,
//         )?));
//         let v = Value {
//             value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//                 seconds: 1_i64,
//                 nanos: 2_i32,
//             })),
//         };
//         let s = to_value(&o)?;
//         let d = from_value::<'_, S>(&s)?;
//         assert_eq!(s, v);
//         assert_eq!(d, o);

//         let o = S(None);
//         let v = Value {
//             value_type: Some(ValueType::NullValue(0)),
//         };
//         let s = to_value(&o)?;
//         let d = from_value::<'_, S>(&s)?;
//         assert_eq!(s, v);
//         assert_eq!(d, o);
//         Ok(())
//     }
// }
