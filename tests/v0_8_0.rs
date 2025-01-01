#![allow(missing_docs)]

#[test]
fn test_impl_ord_for_timestamp() -> serde_firestore_value::Result<()> {
    use serde_firestore_value::Timestamp;
    let timestamp1 = Timestamp {
        seconds: 1_i64,
        nanos: 2_i32,
    };
    let timestamp2 = Timestamp {
        seconds: 1_i64,
        nanos: 3_i32,
    };
    let timestamp3 = Timestamp {
        seconds: 2_i64,
        nanos: 1_i32,
    };
    assert_eq!(timestamp1.cmp(&timestamp2), std::cmp::Ordering::Less);
    assert_eq!(timestamp1.cmp(&timestamp1), std::cmp::Ordering::Equal);
    assert_eq!(timestamp2.cmp(&timestamp1), std::cmp::Ordering::Greater);
    assert_eq!(timestamp2.cmp(&timestamp3), std::cmp::Ordering::Less);
    assert_eq!(timestamp1.cmp(&timestamp3), std::cmp::Ordering::Less);
    fn assert_fn<T: std::cmp::Ord>() {}
    assert_fn::<Timestamp>();
    Ok(())
}
