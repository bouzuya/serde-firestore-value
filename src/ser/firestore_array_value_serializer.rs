use crate::google::firestore::v1::Value;
use crate::{ser::Error, value_ext::ValueExt};

use super::firestore_value_serializer::Serializer;

#[doc(hidden)]
pub struct FirestoreArrayValueSerializer {
    values: Vec<Value>,
}

impl FirestoreArrayValueSerializer {
    pub(crate) fn new(len: Option<usize>) -> Self {
        Self {
            values: Vec::with_capacity(len.unwrap_or(0)),
        }
    }
}

impl serde::ser::SerializeSeq for FirestoreArrayValueSerializer {
    type Ok = Value;

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.values.push(value.serialize(Serializer::new())?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from_values(self.values))
    }
}

impl serde::ser::SerializeTuple for FirestoreArrayValueSerializer {
    type Ok = Value;

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl serde::ser::SerializeTupleStruct for FirestoreArrayValueSerializer {
    type Ok = Value;

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}
impl serde::ser::SerializeTupleVariant for FirestoreArrayValueSerializer {
    type Ok = Value;

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}
