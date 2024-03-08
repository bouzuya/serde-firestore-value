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
