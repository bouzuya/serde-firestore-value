use google::r#type::LatLng;

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<LatLng>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::de::with::lat_lng_as_geo_point::deserialize_option_lat_lng(deserializer)
}

pub fn serialize<S>(lat_lng: &Option<LatLng>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    crate::ser::with::lat_lng_as_geo_point::serialize_option_lat_lng(lat_lng, serializer)
}
