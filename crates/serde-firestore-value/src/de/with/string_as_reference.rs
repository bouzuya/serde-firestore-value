use serde::Deserialize;

use crate::typ::my_reference::MyReference;

pub(crate) fn deserialize_string_as_reference<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    MyReference::deserialize(deserializer).map(String::from)
}

pub(crate) fn deserialize_option_string_as_reference<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<MyReference>::deserialize(deserializer).map(|o| o.map(String::from))
}
