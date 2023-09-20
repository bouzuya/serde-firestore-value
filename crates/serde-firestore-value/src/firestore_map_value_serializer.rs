use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, MapValue, Value};

use crate::{firestore_value_serializer::FirestoreValueSerializer, to_value, Error, ErrorCode};

pub(crate) struct FirestoreMapValueSerializer<'a> {
    key: Option<String>,
    name: Option<&'static str>,
    output: MapValue,
    parent: &'a mut FirestoreValueSerializer,
}

impl<'a> FirestoreMapValueSerializer<'a> {
    pub(crate) fn new(
        parent: &'a mut FirestoreValueSerializer,
        name: Option<&'static str>,
        len: Option<usize>,
    ) -> Self {
        Self {
            key: None,
            name,
            output: MapValue {
                fields: HashMap::with_capacity(len.unwrap_or(0)),
            },
            parent,
        }
    }
}

impl<'a> serde::ser::SerializeMap for FirestoreMapValueSerializer<'a> {
    type Ok = &'a mut FirestoreValueSerializer;

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        if let Value {
            value_type: Some(ValueType::StringValue(key_string)),
        } = to_value(&key)?
        {
            if self.key.is_none() {
                self.key = Some(key_string);
                Ok(())
            } else {
                unreachable!()
            }
        } else {
            Err(Error::from(ErrorCode::KeyMustBeAString))
        }
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        if let Some(k) = self.key.take() {
            let v = to_value(&value)?;
            self.output.fields.insert(k, v);
            Ok(())
        } else {
            unreachable!()
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.parent.set_map_value(self.name, self.output);
        Ok(self.parent)
    }
}

impl<'a> serde::ser::SerializeStruct for FirestoreMapValueSerializer<'a> {
    type Ok = &'a mut FirestoreValueSerializer;

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        serde::ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeMap::end(self)
    }
}
impl<'a> serde::ser::SerializeStructVariant for FirestoreMapValueSerializer<'a> {
    type Ok = &'a mut FirestoreValueSerializer;

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.output
            .fields
            .insert(key.to_string(), to_value(&value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.parent.set_map_value(self.name, self.output);
        Ok(self.parent)
    }
}
