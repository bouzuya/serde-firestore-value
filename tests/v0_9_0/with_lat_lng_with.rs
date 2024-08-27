#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::{
        firestore::v1::{value::ValueType, Value},
        r#type::LatLng,
    };
    use serde_firestore_value::{from_value, to_value, with::lat_lng};

    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    struct S(#[serde(with = "lat_lng")] LatLng);
    let o = S(LatLng {
        latitude: 1_f64,
        longitude: 2_f64,
    });
    let v = Value {
        value_type: Some(ValueType::GeoPointValue(LatLng {
            latitude: 1_f64,
            longitude: 2_f64,
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, S>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);
    Ok(())
}
