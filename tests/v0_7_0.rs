#![allow(missing_docs)]

#[cfg(feature = "chrono")]
#[test]
fn test_try_from_timestamp_for_chrono_date_time() -> serde_firestore_value::Result<()> {
    use serde_firestore_value::Timestamp;
    let timestamp = Timestamp {
        seconds: 1_i64,
        nanos: 2_i32,
    };
    let date_time = chrono::DateTime::<chrono::Utc>::try_from(timestamp)?;
    assert_eq!(
        date_time.to_rfc3339(),
        "1970-01-01T00:00:01.000000002+00:00"
    );
    Ok(())
}

#[cfg(feature = "time")]
#[test]
fn test_try_from_timestamp_for_time_offset_date_time() -> serde_firestore_value::Result<()> {
    use serde_firestore_value::Timestamp;
    let timestamp = Timestamp {
        seconds: 1_i64,
        nanos: 2_i32,
    };
    let offset_date_time = time::OffsetDateTime::try_from(timestamp)?;
    assert_eq!(offset_date_time.unix_timestamp_nanos(), 1_000_000_002_i128);
    assert_eq!(
        offset_date_time.to_string(),
        "1970-01-01 0:00:01.000000002 +00:00:00"
    );
    Ok(())
}

#[cfg(feature = "chrono")]
#[test]
fn test_try_from_chrono_date_time_for_timestamp() -> anyhow::Result<()> {
    use serde_firestore_value::Timestamp;
    let date_time = chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
        "1970-01-01T00:00:01.000000002+00:00",
    )?
    .naive_utc()
    .and_utc();
    let timestamp = Timestamp::try_from(date_time)?;
    assert_eq!(
        timestamp,
        Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        }
    );
    Ok(())
}

#[cfg(feature = "time")]
#[test]
fn test_try_from_time_offset_date_time_for_timestamp() -> anyhow::Result<()> {
    use serde_firestore_value::Timestamp;
    let offset_date_time = time::OffsetDateTime::from_unix_timestamp_nanos(1_000_000_002_i128)?;
    let timestamp = Timestamp::try_from(offset_date_time)?;
    assert_eq!(
        timestamp,
        Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        }
    );
    Ok(())
}
