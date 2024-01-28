use crate::Reference;

pub(crate) fn serialize_string_as_reference<S>(
    value: &str,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serde::Serialize::serialize(&Reference::from(value.to_string()), serializer)
}

pub(crate) fn serialize_option_string_as_reference<S>(
    value: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serde::Serialize::serialize(&value.as_ref().cloned().map(Reference::from), serializer)
}

pub(crate) fn serialize_vec_string_as_reference<S>(
    value: &[String],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serde::Serialize::serialize(
        &value
            .iter()
            .cloned()
            .map(Reference::from)
            .collect::<Vec<Reference>>(),
        serializer,
    )
}
