use google_api_proto::google::firestore::v1::Value;

use crate::{value_ext::ValueExt, Error};

use super::{
    firestore_array_value_deserializer::FirestoreArrayValueDeserializer,
    firestore_map_value_deserializer::FirestoreMapValueDeserializer, FirestoreValueDeserializer,
};

pub(super) struct FirestoreEnumDeserializer<'de> {
    value: &'de Value,
    variants: &'static [&'static str],
}

impl<'de> FirestoreEnumDeserializer<'de> {
    pub(super) fn new(value: &'de Value, variants: &'static [&'static str]) -> Result<Self, Error> {
        Ok(Self { value, variants })
    }
}

impl<'de> serde::de::EnumAccess<'de> for FirestoreEnumDeserializer<'de> {
    type Error = Error;
    type Variant = FirestoreEnumDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(FirestoreValueDeserializer::new(self.value))
            .map(|v| (v, self))
    }
}

impl<'de> serde::de::VariantAccess<'de> for FirestoreEnumDeserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        let variant_name = self.value.as_string()?;
        if self.variants.contains(&variant_name.as_str()) {
            Ok(())
        } else {
            Err(<Error as serde::de::Error>::unknown_variant(
                variant_name.as_str(),
                self.variants,
            ))
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let value = self.value.as_variant_value(self.variants)?;
        seed.deserialize(FirestoreValueDeserializer::new(value))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value = self.value.as_variant_value(self.variants)?;
        visitor.visit_seq(FirestoreArrayValueDeserializer::new(value)?)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value = self.value.as_variant_value(self.variants)?;
        visitor.visit_map(FirestoreMapValueDeserializer::new(value)?)
    }
}
