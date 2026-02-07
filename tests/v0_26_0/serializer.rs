#[test]
fn test() -> serde_firestore_value::Result<()> {
    use serde::Serialize;
    use serde_firestore_value::{Serializer, to_value};
    #[derive(serde::Serialize)]
    struct T;

    let o = T;
    assert_eq!(o.serialize(Serializer::new())?, to_value(&o)?);
    Ok(())
}
