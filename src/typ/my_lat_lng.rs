use google_api_proto::google::r#type::LatLng as GoogleApiProtoLatLng;

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "$__serde-firestore-value_private_lat_lng")]
pub(crate) struct MyLatLng {
    latitude: f64,
    longitude: f64,
}

impl MyLatLng {
    pub(crate) const NAME: &'static str = "$__serde-firestore-value_private_lat_lng";
}

impl From<GoogleApiProtoLatLng> for MyLatLng {
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

impl From<MyLatLng> for GoogleApiProtoLatLng {
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
