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
