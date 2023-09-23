use google::r#type::LatLng;

pub fn deserialize<'de, D>(deserializer: D) -> Result<LatLng, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::de::lat_lng::deserialize_lat_lng(deserializer)
}

pub fn serialize<S>(lat_lng: &LatLng, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::ser::lat_lng::serialize_lat_lng(lat_lng, serializer)
}
