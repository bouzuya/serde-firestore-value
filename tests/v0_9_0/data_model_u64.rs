#[test]
fn test() -> anyhow::Result<()> {
    use serde_firestore_value::to_value;

    let o = u64::MAX;
    assert_eq!(
        to_value(&o).unwrap_err().to_string(),
        "u64 is not supported"
    );

    let o = u64::MIN;
    assert_eq!(
        to_value(&o).unwrap_err().to_string(),
        "u64 is not supported"
    );
    Ok(())
}
