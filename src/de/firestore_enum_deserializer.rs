use crate::google::firestore::v1::Value;
use crate::{Error, value_ext::ValueExt};

use super::{Deserializer, firestore_array_value_deserializer::FirestoreArrayValueDeserializer};

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
        seed.deserialize(Deserializer::new(self.value))
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
        let (variant, value) = self.value.as_variant_value()?;
        if self.variants.contains(&variant.as_str()) {
            seed.deserialize(Deserializer::new(value))
        } else {
            Err(<Error as serde::de::Error>::unknown_variant(
                variant,
                self.variants,
            ))
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let (variant, value) = self.value.as_variant_value()?;
        if self.variants.contains(&variant.as_str()) {
            visitor.visit_seq(FirestoreArrayValueDeserializer::new(value)?)
        } else {
            Err(<Error as serde::de::Error>::unknown_variant(
                variant,
                self.variants,
            ))
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let (variant, value) = self.value.as_variant_value()?;
        if self.variants.contains(&variant.as_str()) {
            visitor.visit_map(serde::de::value::MapDeserializer::new(
                value
                    .as_fields()?
                    .iter()
                    .map(|(k, v)| (k.as_str(), Deserializer::new(v))),
            ))
        } else {
            Err(<Error as serde::de::Error>::unknown_variant(
                variant,
                self.variants,
            ))
        }
    }
}
