#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{Value, value::ValueType};
    use serde_firestore_value::{Reference, from_value, to_value};

    let inner = "projects/p/databases/d/documents/c/d";
    let o = Reference(inner.to_string());
    let v = Value {
        value_type: Some(ValueType::ReferenceValue(inner.to_string())),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, Reference>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let json = serde_json::to_string(&o)?;
    assert_eq!(json, r#""projects/p/databases/d/documents/c/d""#);
    Ok(())
}
