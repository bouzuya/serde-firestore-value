#[cfg(feature = "hash-map")]
#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::{
        firestore::v1::{value::ValueType, MapValue, Value},
        r#type::LatLng,
    };
    use serde_firestore_value::{from_value, to_value, with::option_lat_lng};

    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S {
        #[serde(with = "option_lat_lng")]
        a: Option<LatLng>,
        #[serde(with = "option_lat_lng")]
        b: Option<LatLng>,
    }
    let o = S {
        a: Some(LatLng {
            latitude: 1_f64,
            longitude: 2_f64,
        }),
        b: None,
    };
    let v = Value {
        value_type: Some(ValueType::MapValue(MapValue {
            fields: {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    "a".to_string(),
                    Value {
                        value_type: Some(ValueType::GeoPointValue(LatLng {
                            latitude: 1_f64,
                            longitude: 2_f64,
                        })),
                    },
                );
                map.insert(
                    "b".to_string(),
                    Value {
                        value_type: Some(ValueType::NullValue(0_i32)),
                    },
                );
                map
            },
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, S>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
