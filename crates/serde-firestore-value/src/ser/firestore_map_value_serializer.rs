use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, Value};

use crate::{ser::Error, value_ext::ValueExt};

use super::{error::ErrorCode, firestore_value_serializer::FirestoreValueSerializer};

pub(crate) struct FirestoreMapValueSerializer {
    fields: HashMap<String, Value>,
    key: Option<String>,
}

impl FirestoreMapValueSerializer {
    pub(crate) fn new(len: Option<usize>) -> Self {
        Self {
            fields: HashMap::with_capacity(len.unwrap_or(0)),
            key: None,
        }
    }
}

impl serde::ser::SerializeMap for FirestoreMapValueSerializer {
    type Ok = Value;

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        if let Value {
            value_type: Some(ValueType::StringValue(key_string)),
        } = key.serialize(FirestoreValueSerializer)?
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
            let v = value.serialize(FirestoreValueSerializer)?;
            self.fields.insert(k, v);
            Ok(())
        } else {
            unreachable!()
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from_fields(self.fields))
    }
}

impl serde::ser::SerializeStruct for FirestoreMapValueSerializer {
    type Ok = Value;

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
impl serde::ser::SerializeStructVariant for FirestoreMapValueSerializer {
    type Ok = Value;

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
