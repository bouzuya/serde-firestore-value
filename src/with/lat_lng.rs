//! (De)serialize `LatLng` as `geoPointValue`.

use crate::google::r#type::LatLng;

/// Deserialize `LatLng` from `geoPointValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use googleapis_tonic_google_firestore_v1::google::{
///     firestore::v1::{value::ValueType, Value},
///     r#type::LatLng,
/// };
/// use serde_firestore_value::{from_value, with::lat_lng};
///
/// #[derive(Debug, PartialEq, serde::Deserialize)]
/// struct S(#[serde(deserialize_with = "lat_lng::deserialize")] LatLng);
/// let o = S(LatLng {
///     latitude: 1_f64,
///     longitude: 2_f64,
/// });
/// let v = Value {
///     value_type: Some(ValueType::GeoPointValue(LatLng {
///         latitude: 1_f64,
///         longitude: 2_f64,
///     })),
/// };
/// let d = from_value::<'_, S>(&v)?;
/// assert_eq!(d, o);
/// #     Ok(())
/// # }
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<LatLng, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::de::with::lat_lng_as_geo_point::deserialize_lat_lng(deserializer)
}

/// Serialize `LatLng` as `geoPointValue`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use googleapis_tonic_google_firestore_v1::google::{
///     firestore::v1::{value::ValueType, Value},
///     r#type::LatLng,
/// };
/// use serde_firestore_value::{to_value, with::lat_lng};
///
/// #[derive(Debug, PartialEq, serde::Serialize)]
/// struct S(#[serde(serialize_with = "lat_lng::serialize")] LatLng);
/// let o = S(LatLng {
///     latitude: 1_f64,
///     longitude: 2_f64,
/// });
/// let v = Value {
///     value_type: Some(ValueType::GeoPointValue(LatLng {
///         latitude: 1_f64,
///         longitude: 2_f64,
///     })),
/// };
/// let s = to_value(&o)?;
/// assert_eq!(s, v);
/// #     Ok(())
/// # }
/// ```
pub fn serialize<S>(lat_lng: &LatLng, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::ser::with::lat_lng_as_geo_point::serialize_lat_lng(lat_lng, serializer)
}
