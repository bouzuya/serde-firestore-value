use crate::typ::my_reference::MyReference;

pub(crate) fn serialize_string_as_reference<S>(
    value: &str,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let reference = MyReference::from(value.to_string());
    serde::Serialize::serialize(&reference, serializer)
}

pub(crate) fn serialize_option_string_as_reference<S>(
    value: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(s) => serialize_string_as_reference(s, serializer),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn serialize_vec_string_as_reference<S>(
    value: &[String],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.collect_seq(value.iter().cloned().map(MyReference::from))
}
