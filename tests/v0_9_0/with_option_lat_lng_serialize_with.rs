#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::{
        firestore::v1::{value::ValueType, Value},
        r#type::LatLng,
    };
    use serde_firestore_value::{to_value, with::option_lat_lng};

    #[derive(Debug, PartialEq, serde::Serialize)]
    struct S(#[serde(serialize_with = "option_lat_lng::serialize")] Option<LatLng>);

    // some
    {
        let o = S(Some(LatLng {
            latitude: 1_f64,
            longitude: 2_f64,
        }));
        let v = Value {
            value_type: Some(ValueType::GeoPointValue(LatLng {
                latitude: 1_f64,
                longitude: 2_f64,
            })),
        };
        let s = to_value(&o)?;
        assert_eq!(s, v);
    }

    // none
    {
        let o = S(None);
        let v = Value {
            value_type: Some(ValueType::NullValue(0)),
        };
        let s = to_value(&o)?;
        assert_eq!(s, v);
    }
    Ok(())
}
