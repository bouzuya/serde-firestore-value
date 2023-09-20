use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, MapValue, Value};

use crate::{firestore_named_array_value_serializer::SetMapValue, to_value, Error, ErrorCode};

pub(crate) struct FirestoreMapValueSerializer<'a, S> {
    key: Option<String>,
    output: MapValue,
    parent: &'a mut S,
}

impl<'a, S: SetMapValue> FirestoreMapValueSerializer<'a, S> {
    pub(crate) fn new(parent: &'a mut S, len: Option<usize>) -> Self {
        Self {
            key: None,
            output: MapValue {
                fields: HashMap::with_capacity(len.unwrap_or(0)),
            },
            parent,
        }
    }
}

impl<'a, S: SetMapValue> serde::ser::SerializeMap for FirestoreMapValueSerializer<'a, S> {
    type Ok = &'a mut S;

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
        self.parent.set_map_value(self.output);
        Ok(self.parent)
    }
}

impl<'a, S: SetMapValue> serde::ser::SerializeStruct for FirestoreMapValueSerializer<'a, S> {
    type Ok = &'a mut S;

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
