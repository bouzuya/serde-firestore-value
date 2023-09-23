use std::collections::HashMap;

use google::firestore::v1::{ArrayValue, MapValue, Value};
use prost_types::Timestamp;
use serde::{ser::SerializeMap, Serialize, Serializer};

use crate::{
    serializer::{
        firestore_array_value_serializer::FirestoreArrayValueSerializer,
        firestore_map_value_serializer::FirestoreMapValueSerializer,
    },
    value_ext::ValueExt,
};

use super::{
    error::ErrorCode, firestore_timestamp_value_serializer::FirestoreTimestampValueSerializer,
    firestore_value_struct_serializer::FirestoreValueStructSerializer, Error,
};

#[derive(Debug, Default)]
pub(crate) struct FirestoreValueSerializer {
    output: Value,
}

impl FirestoreValueSerializer {
    pub(crate) fn into_inner(self) -> Value {
        self.output
    }

    pub(crate) fn set_array_value(&mut self, name: Option<&'static str>, value: ArrayValue) {
        self.output = match name {
            Some(name) => Value::from_fields({
                let mut fields = HashMap::new();
                fields.insert(name.to_string(), Value::from_array_value(value));
                fields
            }),
            None => Value::from_array_value(value),
        };
    }

    pub(crate) fn set_map_value(&mut self, name: Option<&'static str>, value: MapValue) {
        self.output = match name {
            Some(name) => Value::from_fields({
                let mut fields = HashMap::new();
                fields.insert(name.to_string(), Value::from_map_value(value));
                fields
            }),
            None => Value::from_map_value(value),
        };
    }

    pub(crate) fn set_timestamp_value(&mut self, name: Option<&'static str>, value: Timestamp) {
        self.output = match name {
            Some(name) => Value::from_fields({
                let mut fields = HashMap::new();
                fields.insert(name.to_string(), Value::from_timestamp(value));
                fields
            }),
            None => Value::from_timestamp(value),
        };
    }
}

// 1,048,487 bytes = 1MiB - 89 bytes
const MAX_BYTE_LEN: usize = 1_048_487;

impl<'a> Serializer for &'a mut FirestoreValueSerializer {
    type Ok = &'a mut FirestoreValueSerializer;

    type Error = Error;

    type SerializeSeq = FirestoreArrayValueSerializer<'a>;

    type SerializeTuple = FirestoreArrayValueSerializer<'a>;

    type SerializeTupleStruct = FirestoreArrayValueSerializer<'a>;

    type SerializeTupleVariant = FirestoreArrayValueSerializer<'a>;

    type SerializeMap = FirestoreMapValueSerializer<'a>;

    type SerializeStruct = FirestoreValueStructSerializer<'a>;

    type SerializeStructVariant = FirestoreMapValueSerializer<'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output = Value::from_bool(v);
        Ok(self)
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
        self.output = Value::from_i64(v);
        Ok(self)
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
        self.output = Value::from_f64(v);
        Ok(self)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        if v.len() > MAX_BYTE_LEN {
            return Err(Error::from(ErrorCode::MaximumByteLengthExceeded));
        }
        self.output = Value::from_string(v.to_string());
        Ok(self)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        // NOTE: unreachable. See: <https://serde.rs/impl-serialize.html#other-special-cases>
        if v.len() > MAX_BYTE_LEN {
            return Err(Error::from(ErrorCode::MaximumByteLengthExceeded));
        }
        self.output = Value::from_bytes(v.to_vec());
        Ok(self)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.output = Value::null();
        Ok(self)
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
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
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
        Ok(FirestoreArrayValueSerializer::new(self, None, len))
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
        Ok(FirestoreArrayValueSerializer::new(
            self,
            Some(variant),
            Some(len),
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(FirestoreMapValueSerializer::new(self, None, len))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(if name == FirestoreTimestampValueSerializer::NAME {
            FirestoreValueStructSerializer::Timestamp(FirestoreTimestampValueSerializer::new(self))
        } else {
            FirestoreValueStructSerializer::MapValue(FirestoreMapValueSerializer::new(
                self,
                None,
                Some(len),
            ))
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(FirestoreMapValueSerializer::new(
            self,
            Some(variant),
            Some(len),
        ))
    }
}
