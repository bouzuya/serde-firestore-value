#[cfg(feature = "hash-map")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        MapValue, Value, value::ValueType,
    };
    use serde_firestore_value::{from_value, to_value};

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    enum E {
        N(u8),
    }

    let o = E::N(u8::MAX);
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: [(
                "N".to_owned(),
                Value {
                    value_type: Some(ValueType::IntegerValue(i64::from(u8::MAX))),
                },
            )]
            .into_iter()
            .collect::<std::collections::HashMap<String, Value>>(),
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, E>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
