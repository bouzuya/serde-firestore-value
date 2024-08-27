#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::{
        firestore::v1::{value::ValueType, Value},
        r#type::LatLng,
    };
    use serde_firestore_value::{from_value, with::lat_lng};

    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct S(#[serde(deserialize_with = "lat_lng::deserialize")] LatLng);
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
    let d = from_value::<'_, S>(&v)?;
    assert_eq!(d, o);
    Ok(())
}
