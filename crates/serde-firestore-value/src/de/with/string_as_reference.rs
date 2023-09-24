use serde::Deserialize;

pub fn deserialize_string_as_reference<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Debug, serde::Deserialize)]
    #[serde(rename = "$__serde-firestore-value_private_string_as_reference")]
    struct S(String);
    S::deserialize(deserializer).map(|S(s)| s)
}
