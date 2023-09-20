use google::firestore::v1::ArrayValue;

use crate::{serializer::firestore_value_serializer::FirestoreValueSerializer, to_value, Error};

pub(crate) struct FirestoreArrayValueSerializer<'a> {
    name: Option<&'static str>,
    output: ArrayValue,
    parent: &'a mut FirestoreValueSerializer,
}

impl<'a> FirestoreArrayValueSerializer<'a> {
    pub(crate) fn new(
        parent: &'a mut FirestoreValueSerializer,
        name: Option<&'static str>,
        len: Option<usize>,
    ) -> Self {
        Self {
            name,
            output: ArrayValue {
                values: Vec::with_capacity(len.unwrap_or(0)),
            },
            parent,
        }
    }
}

impl<'a> serde::ser::SerializeSeq for FirestoreArrayValueSerializer<'a> {
    type Ok = &'a mut FirestoreValueSerializer;

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.output.values.push(to_value(&value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.parent.set_array_value(self.name, self.output);
        Ok(self.parent)
    }
}

impl<'a> serde::ser::SerializeTuple for FirestoreArrayValueSerializer<'a> {
    type Ok = &'a mut FirestoreValueSerializer;

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'a> serde::ser::SerializeTupleStruct for FirestoreArrayValueSerializer<'a> {
    type Ok = &'a mut FirestoreValueSerializer;

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}
impl<'a> serde::ser::SerializeTupleVariant for FirestoreArrayValueSerializer<'a> {
    type Ok = &'a mut FirestoreValueSerializer;

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}
