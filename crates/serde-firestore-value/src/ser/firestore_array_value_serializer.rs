use std::collections::HashMap;

use google::firestore::v1::{ArrayValue, Value};

use crate::{ser::Error, value_ext::ValueExt};

use super::firestore_value_serializer::FirestoreValueSerializer;

pub(crate) struct FirestoreArrayValueSerializer {
    name: Option<&'static str>,
    output: ArrayValue,
}

impl FirestoreArrayValueSerializer {
    pub(crate) fn new(name: Option<&'static str>, len: Option<usize>) -> Self {
        Self {
            name,
            output: ArrayValue {
                values: Vec::with_capacity(len.unwrap_or(0)),
            },
        }
    }
}

impl serde::ser::SerializeSeq for FirestoreArrayValueSerializer {
    type Ok = Value;

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.output
            .values
            .push(value.serialize(FirestoreValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(match self.name {
            Some(name) => Value::from_fields({
                let mut fields = HashMap::new();
                fields.insert(name.to_string(), Value::from_array_value(self.output));
                fields
            }),
            None => Value::from_array_value(self.output),
        })
    }
}

impl serde::ser::SerializeTuple for FirestoreArrayValueSerializer {
    type Ok = Value;

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

impl serde::ser::SerializeTupleStruct for FirestoreArrayValueSerializer {
    type Ok = Value;

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
impl serde::ser::SerializeTupleVariant for FirestoreArrayValueSerializer {
    type Ok = Value;

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
