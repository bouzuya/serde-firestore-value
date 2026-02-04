use serde::forward_to_deserialize_any;

use crate::google::firestore::v1::Value;
use crate::value_ext::ValueExt;

use super::Error;

pub(super) struct FirestoreFieldReferenceValueDeserializer<'de> {
    value: &'de Value,
}

impl<'de> FirestoreFieldReferenceValueDeserializer<'de> {
    pub(super) fn new(value: &'de Value) -> Self {
        Self { value }
    }
}

impl<'de> serde::de::Deserializer<'de> for FirestoreFieldReferenceValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_str(self.value.as_field_reference_value_as_string()?)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
