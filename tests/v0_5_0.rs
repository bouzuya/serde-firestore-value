#[test]
fn test_lat_lng() -> anyhow::Result<()> {
    use google_api_proto::google::firestore::v1::{value::ValueType, Value};
    use serde_firestore_value::{from_value, to_value, LatLng};

    let o = LatLng {
        latitude: 1_f64,
        longitude: 2_f64,
    };
    let v = Value {
        value_type: Some(ValueType::GeoPointValue(
            google_api_proto::google::r#type::LatLng {
                latitude: 1_f64,
                longitude: 2_f64,
            },
        )),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, LatLng>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let json = serde_json::to_string(&o)?;
    assert_eq!(json, r#"{"latitude":1.0,"longitude":2.0}"#);
    Ok(())
}
