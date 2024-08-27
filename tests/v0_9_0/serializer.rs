#[test]
fn test() -> serde_firestore_value::Result<()> {
    use serde::Serialize;
    use serde_firestore_value::{to_value, Serializer};
    #[derive(serde::Serialize)]
    struct T;

    let o = T;
    assert_eq!(o.serialize(Serializer)?, to_value(&o)?);
    Ok(())
}
