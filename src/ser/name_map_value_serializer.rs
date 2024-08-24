use crate::google::firestore::v1::Value;
use crate::value_ext::ValueExt;

#[doc(hidden)]
pub struct NameMapValueSerializer<S> {
    name: &'static str,
    serializer: S,
}

impl<S> NameMapValueSerializer<S> {
    pub(crate) fn new(name: &'static str, serializer: S) -> Self {
        Self { name, serializer }
    }
}

impl<S: serde::ser::SerializeTupleVariant<Ok = Value>> serde::ser::SerializeTupleVariant
    for NameMapValueSerializer<S>
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.serializer.serialize_field(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let value = self.serializer.end()?;
        Ok(Self::Ok::from_fields([(self.name, value)]))
    }
}

impl<S: serde::ser::SerializeStructVariant<Ok = Value>> serde::ser::SerializeStructVariant
    for NameMapValueSerializer<S>
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.serializer.serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let value = self.serializer.end()?;
        Ok(Self::Ok::from_fields([(self.name, value)]))
    }
}
