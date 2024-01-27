#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "$__serde-firestore-value_private_timestamp")]
pub(crate) struct MyTimestamp {
    seconds: i64,
    nanos: i32,
}

impl MyTimestamp {
    pub(crate) const NAME: &'static str = "$__serde-firestore-value_private_timestamp";
}

impl From<MyTimestamp> for prost_types::Timestamp {
    fn from(MyTimestamp { seconds, nanos }: MyTimestamp) -> Self {
        Self { seconds, nanos }
    }
}

impl From<prost_types::Timestamp> for MyTimestamp {
    fn from(prost_types::Timestamp { seconds, nanos }: prost_types::Timestamp) -> Self {
        Self { seconds, nanos }
    }
}
