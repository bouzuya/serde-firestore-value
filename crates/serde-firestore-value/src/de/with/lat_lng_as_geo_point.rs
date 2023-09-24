use google::r#type::LatLng;
use serde::Deserialize;

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "$__serde-firestore-value_private_lat_lng")]
struct MyLatLng {
    latitude: f64,
    longitude: f64,
}

impl From<MyLatLng> for LatLng {
    fn from(
        MyLatLng {
            latitude,
            longitude,
        }: MyLatLng,
    ) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

pub fn deserialize_lat_lng<'de, D>(deserializer: D) -> Result<LatLng, D::Error>
where
    D: serde::Deserializer<'de>,
{
    MyLatLng::deserialize(deserializer).map(LatLng::from)
}

pub fn deserialize_option_lat_lng<'de, D>(deserializer: D) -> Result<Option<LatLng>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<MyLatLng>::deserialize(deserializer).map(|o| o.map(LatLng::from))
}
