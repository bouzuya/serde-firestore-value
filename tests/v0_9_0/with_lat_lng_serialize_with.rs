#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::{
        firestore::v1::{Value, value::ValueType},
        r#type::LatLng,
    };
    use serde_firestore_value::{to_value, with::lat_lng};

    #[derive(Debug, PartialEq, serde::Serialize)]
    struct S(#[serde(serialize_with = "lat_lng::serialize")] LatLng);
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
    assert_eq!(s, v);
    Ok(())
}
