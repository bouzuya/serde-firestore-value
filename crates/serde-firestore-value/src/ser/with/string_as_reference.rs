use super::super::firestore_value_serializer::FirestoreValueSerializer;

pub(crate) fn serialize_string_as_reference<S>(
    value: &String,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_newtype_struct(
        FirestoreValueSerializer::STRING_AS_REFERENCE_STRUCT_NAME,
        value,
    )
}
