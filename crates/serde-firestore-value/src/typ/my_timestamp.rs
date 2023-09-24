use prost_types::Timestamp;

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "$__serde-firestore-value_private_timestamp")]
pub(crate) struct MyTimestamp {
    seconds: i64,
    nanos: i32,
}

impl MyTimestamp {
    pub(crate) const NAME: &'static str = "$__serde-firestore-value_private_timestamp";
}

impl From<MyTimestamp> for Timestamp {
    fn from(MyTimestamp { seconds, nanos }: MyTimestamp) -> Self {
        Self { seconds, nanos }
    }
}

impl From<Timestamp> for MyTimestamp {
    fn from(Timestamp { seconds, nanos }: Timestamp) -> Self {
        Self { seconds, nanos }
    }
}
