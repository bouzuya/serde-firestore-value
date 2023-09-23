use google::{
    firestore::v1::{value::ValueType, Value},
    r#type::LatLng,
};
use serde_firestore_value::{from_value, lat_lng, to_value};

#[test]
fn test_deserialize_with() -> anyhow::Result<()> {
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

#[test]
fn test_serialize_with() -> anyhow::Result<()> {
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

#[test]
fn test_newtype_struct() -> anyhow::Result<()> {
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
