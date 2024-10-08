#[test]
fn test() -> Result<(), serde_firestore_value::Error> {
    // re-export ::googleapis_tonic_google_firestore_v1::google module
    use serde_firestore_value::google;
    use serde_firestore_value::to_value;

    assert_eq!(
        to_value(&true)?,
        google::firestore::v1::Value {
            value_type: Some(google::firestore::v1::value::ValueType::BooleanValue(true))
        }
    );
    Ok(())
}
