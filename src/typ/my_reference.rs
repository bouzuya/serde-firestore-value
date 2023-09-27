#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "$__serde-firestore-value_private_string_as_reference")]
pub(crate) struct MyReference(String);

impl MyReference {
    pub(crate) const NAME: &str = "$__serde-firestore-value_private_string_as_reference";
}

impl From<MyReference> for String {
    fn from(MyReference(s): MyReference) -> Self {
        s
    }
}

impl From<String> for MyReference {
    fn from(s: String) -> Self {
        Self(s)
    }
}
