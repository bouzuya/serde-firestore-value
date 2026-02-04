use serde::Deserialize;

use crate::FieldReference;

pub(crate) fn deserialize_string_as_field_reference<'de, D>(
    deserializer: D,
) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    FieldReference::deserialize(deserializer).map(String::from)
}
