use google::r#type::LatLng;
use serde::Deserialize;

use crate::typ::my_lat_lng::MyLatLng;

pub(crate) fn deserialize_lat_lng<'de, D>(deserializer: D) -> Result<LatLng, D::Error>
where
    D: serde::Deserializer<'de>,
{
    MyLatLng::deserialize(deserializer).map(LatLng::from)
}

pub(crate) fn deserialize_option_lat_lng<'de, D>(
    deserializer: D,
) -> Result<Option<LatLng>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<MyLatLng>::deserialize(deserializer).map(|o| o.map(LatLng::from))
}
