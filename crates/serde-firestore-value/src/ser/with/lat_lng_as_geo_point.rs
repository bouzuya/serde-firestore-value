use google::r#type::LatLng;

use super::super::firestore_value_serializer::FirestoreValueSerializer;

pub fn serialize_lat_lng<S>(lat_lng: &LatLng, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut s = serializer.serialize_struct(FirestoreValueSerializer::LAT_LNG_STRUCT_NAME, 2)?;
    serde::ser::SerializeStruct::serialize_field(&mut s, "latitude", &lat_lng.latitude)?;
    serde::ser::SerializeStruct::serialize_field(&mut s, "longitude", &lat_lng.longitude)?;
    serde::ser::SerializeStruct::end(s)
}

pub fn serialize_option_lat_lng<S>(
    lat_lng: &Option<LatLng>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match lat_lng {
        Some(lat_lng) => serialize_lat_lng(lat_lng, serializer),
        None => serializer.serialize_none(),
    }
}
