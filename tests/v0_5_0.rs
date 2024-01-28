use serde::{de::Error, Serialize};

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

#[test]
fn test_reference() -> anyhow::Result<()> {
    use google_api_proto::google::firestore::v1::{value::ValueType, Value};
    use serde_firestore_value::{from_value, to_value, Reference};

    let inner = "projects/p/databases/d/documents/c/d";
    let o = Reference(inner.to_string());
    let v = Value {
        value_type: Some(ValueType::ReferenceValue(inner.to_string())),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, Reference>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let json = serde_json::to_string(&o)?;
    assert_eq!(json, r#""projects/p/databases/d/documents/c/d""#);
    Ok(())
}

#[test]
fn test_serializer() -> serde_firestore_value::Result<()> {
    use serde_firestore_value::{to_value, Serializer};
    #[derive(serde::Serialize)]
    struct T;

    let o = T;
    assert_eq!(o.serialize(Serializer)?, to_value(&o)?);
    Ok(())
}

#[test]
fn test_result() {
    use serde_firestore_value::{Error, Result};
    #[derive(serde::Serialize)]
    struct T;
    let _ = Result::<T>::Ok(T);
    let _ = Result::<T>::Err(Error::custom("test"));
}

#[test]
fn test_timestamp() -> anyhow::Result<()> {
    use google_api_proto::google::firestore::v1::{value::ValueType, Value};
    use serde_firestore_value::{from_value, to_value, Timestamp};

    let o = Timestamp {
        seconds: 1_i64,
        nanos: 2_i32,
    };
    let v = Value {
        value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
            seconds: 1_i64,
            nanos: 2_i32,
        })),
    };
    let s = to_value(&o)?;
    let d = from_value::<'_, Timestamp>(&s)?;
    assert_eq!(s, v);
    assert_eq!(d, o);

    let json = serde_json::to_string(&o)?;
    assert_eq!(json, r#"{"seconds":1,"nanos":2}"#);
    Ok(())
}
