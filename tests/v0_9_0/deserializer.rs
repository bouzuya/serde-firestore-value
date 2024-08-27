#[test]
fn test() -> serde_firestore_value::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, Value};
    use serde::Deserialize;
    use serde_firestore_value::{from_value, Deserializer};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct T;

    let value = Value {
        value_type: Some(ValueType::NullValue(0)),
    };

    assert_eq!(
        T::deserialize(Deserializer::new(&value))?,
        from_value(&value)?
    );
    Ok(())
}
