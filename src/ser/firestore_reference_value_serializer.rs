use serde::{Serialize, Serializer, ser::Impossible};

use crate::google::firestore::v1::Value;
use crate::{error::ErrorCode, value_ext::ValueExt};

use super::Error;

#[derive(Debug, Default)]
pub(super) struct FirestoreReferenceValueSerializer;

impl Serializer for FirestoreReferenceValueSerializer {
    type Ok = Value;

    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;

    type SerializeTuple = Impossible<Self::Ok, Self::Error>;

    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;

    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;

    type SerializeMap = Impossible<Self::Ok, Self::Error>;

    type SerializeStruct = Impossible<Self::Ok, Self::Error>;

    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from_string_as_reference_value(v.to_string()))
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::from(ErrorCode::ReferenceValueMustBeAString))
    }
}
