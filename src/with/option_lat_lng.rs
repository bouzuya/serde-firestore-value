//! (De)serialize `Option<LatLng>` as `geoPointValue` or `nullValue`.

use google_api_proto::google::r#type::LatLng;

/// Deserialize `Option<LatLng>` from `geoPointValue` or `nullValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// #     use std::collections::BTreeMap;
/// use google_api_proto::google::{
///     firestore::v1::{value::ValueType, MapValue, Value},
///     r#type::LatLng,
/// };
/// use serde_firestore_value::{from_value, with::option_lat_lng};
///
/// #[derive(Debug, PartialEq, serde::Deserialize)]
/// struct S(#[serde(deserialize_with = "option_lat_lng::deserialize")] Option<LatLng>);
///
/// // some
/// {
///     let o = S(Some(LatLng {
///         latitude: 1_f64,
///         longitude: 2_f64,
///     }));
///     let v = Value {
///         value_type: Some(ValueType::GeoPointValue(LatLng {
///             latitude: 1_f64,
///             longitude: 2_f64,
///         })),
///     };
///     let d = from_value::<'_, S>(&v)?;
///     assert_eq!(d, o);
/// }
///
/// // none
/// {
///     let o = S(None);
///     let v = Value {
///         value_type: Some(ValueType::NullValue(0_i32)),
///     };
///     let d = from_value::<'_, S>(&v)?;
///     assert_eq!(d, o);
/// }
/// #     Ok(())
/// # }
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<LatLng>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::de::with::lat_lng_as_geo_point::deserialize_option_lat_lng(deserializer)
}

/// Serialize `Option<LatLng>` as `geoPointValue` or `nullValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// #     use std::collections::BTreeMap;
/// use google_api_proto::google::{
///     firestore::v1::{value::ValueType, MapValue, Value},
///     r#type::LatLng,
/// };
/// use serde_firestore_value::{to_value, with::option_lat_lng};
/// #[derive(Debug, PartialEq, serde::Serialize)]
/// struct S(#[serde(serialize_with = "option_lat_lng::serialize")] Option<LatLng>);
///
/// // some
/// {
///     let o = S(Some(LatLng {
///         latitude: 1_f64,
///         longitude: 2_f64,
///     }));
///     let v = Value {
///         value_type: Some(ValueType::GeoPointValue(LatLng {
///             latitude: 1_f64,
///             longitude: 2_f64,
///         })),
///     };
///     let s = to_value(&o)?;
///     assert_eq!(s, v);
/// }
///
/// // none
/// {
///     let o = S(None);
///     let v = Value {
///         value_type: Some(ValueType::NullValue(0)),
///     };
///     let s = to_value(&o)?;
///     assert_eq!(s, v);
/// }
/// #     Ok(())
/// # }
/// ```
pub fn serialize<S>(lat_lng: &Option<LatLng>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::ser::with::lat_lng_as_geo_point::serialize_option_lat_lng(lat_lng, serializer)
}
