use std::collections::HashMap;

use google::{
    firestore::v1::{value::ValueType, MapValue, Value},
    r#type::LatLng,
};
use serde_firestore_value::{from_value, option_lat_lng, to_value};

#[test]
fn test_deserialize_with() -> anyhow::Result<()> {
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

#[test]
fn test_serialize_with() -> anyhow::Result<()> {
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

#[test]
fn test_struct() -> anyhow::Result<()> {
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
                let mut map = HashMap::new();
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
