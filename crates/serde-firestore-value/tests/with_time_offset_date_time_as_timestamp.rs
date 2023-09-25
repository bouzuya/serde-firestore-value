#[cfg(feature = "time")]
mod time_feature {
    use google::firestore::v1::{value::ValueType, Value};
    use prost_types::Timestamp;
    use serde_firestore_value::{from_value, to_value, with::time_offset_date_time_as_timestamp};

    #[test]
    fn test_deserialize_with() -> anyhow::Result<()> {
        #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
        struct S(
            #[serde(deserialize_with = "time_offset_date_time_as_timestamp::deserialize")]
            time::OffsetDateTime,
        );

        let o = S(time::OffsetDateTime::from_unix_timestamp_nanos(1_000_000_002_i128)?);
        let v = Value {
            value_type: Some(ValueType::TimestampValue(Timestamp {
                seconds: 1_i64,
                nanos: 2_i32,
            })),
        };
        let d = from_value::<'_, S>(&v)?;
        assert_eq!(d, o);
        Ok(())
    }

    #[test]
    fn test_serialize_with() -> anyhow::Result<()> {
        #[derive(Debug, Eq, PartialEq, serde::Serialize)]
        struct S(
            #[serde(serialize_with = "time_offset_date_time_as_timestamp::serialize")]
            time::OffsetDateTime,
        );

        let o = S(time::OffsetDateTime::from_unix_timestamp_nanos(1_000_000_002_i128)?);
        let v = Value {
            value_type: Some(ValueType::TimestampValue(Timestamp {
                seconds: 1_i64,
                nanos: 2_i32,
            })),
        };
        let s = to_value(&o)?;
        assert_eq!(s, v);
        Ok(())
    }

    #[test]
    fn test_with() -> anyhow::Result<()> {
        #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
        struct S(#[serde(with = "time_offset_date_time_as_timestamp")] time::OffsetDateTime);

        let o = S(time::OffsetDateTime::from_unix_timestamp_nanos(1_000_000_002_i128)?);
        let v = Value {
            value_type: Some(ValueType::TimestampValue(Timestamp {
                seconds: 1_i64,
                nanos: 2_i32,
            })),
        };
        let s = to_value(&o)?;
        let d = from_value::<'_, S>(&s)?;
        assert_eq!(s, v);
        assert_eq!(d, o);
        Ok(())
    }
}
