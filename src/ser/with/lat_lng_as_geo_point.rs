use google_api_proto::google::r#type::LatLng as GoogleApiProtoLatLng;

use crate::typ::my_lat_lng::MyLatLng;

pub(crate) fn serialize_lat_lng<S>(
    lat_lng: &GoogleApiProtoLatLng,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let lat_lng = MyLatLng::from(lat_lng.clone());
    serde::Serialize::serialize(&lat_lng, serializer)
}

pub(crate) fn serialize_option_lat_lng<S>(
    lat_lng: &Option<GoogleApiProtoLatLng>,
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
