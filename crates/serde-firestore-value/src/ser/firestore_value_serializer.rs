use google::firestore::v1::{value::ValueType, Value};
use serde::{ser::SerializeMap, Serialize, Serializer};

use crate::{
    error::ErrorCode,
    ser::{
        firestore_array_value_serializer::FirestoreArrayValueSerializer,
        firestore_map_value_serializer::FirestoreMapValueSerializer,
    },
    typ::my_reference::MyReference,
    value_ext::ValueExt,
};

use super::{
    firestore_value_struct_serializer::FirestoreValueStructSerializer,
    name_map_value_serializer::NameMapValueSerializer, Error,
};

#[derive(Debug, Default)]
pub(crate) struct FirestoreValueSerializer;

// 1,048,487 bytes = 1MiB - 89 bytes
const MAX_BYTE_LEN: usize = 1_048_487;

impl Serializer for FirestoreValueSerializer {
    type Ok = Value;

    type Error = Error;

    type SerializeSeq = FirestoreArrayValueSerializer;

    type SerializeTuple = FirestoreArrayValueSerializer;

    type SerializeTupleStruct = FirestoreArrayValueSerializer;

    type SerializeTupleVariant = NameMapValueSerializer<FirestoreArrayValueSerializer>;

    type SerializeMap = FirestoreMapValueSerializer;

    type SerializeStruct = FirestoreValueStructSerializer;

    type SerializeStructVariant = NameMapValueSerializer<FirestoreMapValueSerializer>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from_bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from_i64(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::U64IsNotSupported))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from_f64(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        if v.len() > MAX_BYTE_LEN {
            return Err(Error::from(ErrorCode::MaximumByteLengthExceeded));
        }
        Ok(Value::from_string(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        // NOTE: unreachable. See: <https://serde.rs/impl-serialize.html#other-special-cases>
        if v.len() > MAX_BYTE_LEN {
            return Err(Error::from(ErrorCode::MaximumByteLengthExceeded));
        }
        Ok(Value::from_bytes(v.to_vec()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::null())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let value = value.serialize(self)?;
        if name == MyReference::NAME {
            if let Value {
                value_type: Some(ValueType::StringValue(s)),
            } = value
            {
                Ok(Value::from_string_as_reference_value(s))
            } else {
                Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
            }
        } else {
            Ok(value)
        }
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let mut map = self.serialize_map(Some(1))?;
        map.serialize_entry(variant, value)?;
        SerializeMap::end(map)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(FirestoreArrayValueSerializer::new(len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(NameMapValueSerializer::new(
            variant,
            FirestoreArrayValueSerializer::new(Some(len)),
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(FirestoreMapValueSerializer::new(len))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Self::SerializeStruct::new(name, len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(NameMapValueSerializer::new(
            variant,
            FirestoreMapValueSerializer::new(Some(len)),
        ))
    }
}
