use crate::google::r#type::LatLng as GoogleApiProtoLatLng;

/// LatLng
///
/// `geoPointValue` inner type.
///
/// <https://firebase.google.com/docs/firestore/reference/rest/Shared.Types/LatLng>
/// <https://firebase.google.com/docs/firestore/reference/rest/Shared.Types/ArrayValue#Value>
///
/// # Examples
///
/// ```rust
/// # fn test_lat_lng() -> Result<(), serde_firestore_value::Error> {
/// #     use serde_firestore_value::google::{firestore::v1::{value::ValueType, Value}, self};
/// #     use serde_firestore_value::{from_value, to_value, LatLng};
/// let o = LatLng {
///     latitude: 1_f64,
///     longitude: 2_f64,
/// };
/// let v = Value {
///     value_type: Some(ValueType::GeoPointValue(
///         google::r#type::LatLng {
///             latitude: 1_f64,
///             longitude: 2_f64,
///         },
///     )),
/// };
/// let s = to_value(&o)?;
/// let d = from_value::<'_, LatLng>(&s)?;
/// assert_eq!(s, v);
/// assert_eq!(d, o);
/// #     Ok(())
/// # }
#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "$__serde-firestore-value_private_lat_lng")]
pub struct LatLng {
    /// latitude
    pub latitude: f64,
    /// longitude
    pub longitude: f64,
}

impl LatLng {
    pub(crate) const NAME: &'static str = "$__serde-firestore-value_private_lat_lng";
}

impl From<GoogleApiProtoLatLng> for LatLng {
    fn from(
        GoogleApiProtoLatLng {
            latitude,
            longitude,
        }: GoogleApiProtoLatLng,
    ) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

impl From<LatLng> for GoogleApiProtoLatLng {
    fn from(
        LatLng {
            latitude,
            longitude,
        }: LatLng,
    ) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}
