#[test]
fn test() -> anyhow::Result<()> {
    use googleapis_tonic_google_firestore_v1::google::{
        firestore::v1::{value::ValueType, Value},
        r#type::LatLng,
    };
    use serde_firestore_value::{from_value, with::option_lat_lng};

    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct S(#[serde(deserialize_with = "option_lat_lng::deserialize")] Option<LatLng>);

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
        let d = from_value::<'_, S>(&v)?;
        assert_eq!(d, o);
    }

    // none
    {
        let o = S(None);
        let v = Value {
            value_type: Some(ValueType::NullValue(0_i32)),
        };
        let d = from_value::<'_, S>(&v)?;
        assert_eq!(d, o);
    }
    Ok(())
}
