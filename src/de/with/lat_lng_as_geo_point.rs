use serde::Deserialize;

use crate::google::r#type::LatLng as GoogleApiProtoLatLng;
use crate::LatLng;

pub(crate) fn deserialize_lat_lng<'de, D>(deserializer: D) -> Result<GoogleApiProtoLatLng, D::Error>
where
    D: serde::Deserializer<'de>,
{
    LatLng::deserialize(deserializer).map(GoogleApiProtoLatLng::from)
}

pub(crate) fn deserialize_option_lat_lng<'de, D>(
    deserializer: D,
) -> Result<Option<GoogleApiProtoLatLng>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<LatLng>::deserialize(deserializer).map(|o| o.map(GoogleApiProtoLatLng::from))
}
