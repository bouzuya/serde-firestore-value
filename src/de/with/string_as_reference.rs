use serde::Deserialize;

use crate::typ::reference::Reference;

pub(crate) fn deserialize_string_as_reference<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Reference::deserialize(deserializer).map(String::from)
}

pub(crate) fn deserialize_option_string_as_reference<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<Reference>::deserialize(deserializer).map(|o| o.map(String::from))
}

pub(crate) fn deserialize_vec_string_as_reference<'de, D>(
    deserializer: D,
) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Vec::<Reference>::deserialize(deserializer).map(|o| o.into_iter().map(String::from).collect())
}
