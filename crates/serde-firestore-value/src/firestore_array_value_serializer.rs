use google::firestore::v1::ArrayValue;

use crate::{to_value, Error};

pub(crate) trait SetArrayValue {
    fn set_array_value(&mut self, value: ArrayValue);
}

pub(crate) struct FirestoreArrayValueSerializer<'a, S: SetArrayValue> {
    output: ArrayValue,
    parent: &'a mut S,
}

impl<'a, S: SetArrayValue> FirestoreArrayValueSerializer<'a, S> {
    pub(crate) fn new(parent: &'a mut S, len: Option<usize>) -> Self {
        Self {
            output: ArrayValue {
                values: Vec::with_capacity(len.unwrap_or(0)),
            },
            parent,
        }
    }
}

impl<'a, S: SetArrayValue> serde::ser::SerializeSeq for FirestoreArrayValueSerializer<'a, S> {
    type Ok = &'a mut S;

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.output.values.push(to_value(&value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.parent.set_array_value(self.output);
        Ok(self.parent)
    }
}

impl<'a, S: SetArrayValue> serde::ser::SerializeTuple for FirestoreArrayValueSerializer<'a, S> {
    type Ok = &'a mut S;

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.output.values.push(to_value(&value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.parent.set_array_value(self.output);
        Ok(self.parent)
    }
}

impl<'a, S: SetArrayValue> serde::ser::SerializeTupleStruct
    for FirestoreArrayValueSerializer<'a, S>
{
    type Ok = &'a mut S;

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.output.values.push(to_value(&value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.parent.set_array_value(self.output);
        Ok(self.parent)
    }
}
