use std::collections::BTreeMap;

use crate::google::firestore::v1::{Value, value::ValueType};
use crate::{Error, error::ErrorCode, value_ext::ValueExt};

use super::firestore_value_serializer::Serializer;

#[doc(hidden)]
pub struct FirestoreMapValueSerializer {
    fields: BTreeMap<String, Value>,
    key: Option<String>,
}

impl FirestoreMapValueSerializer {
    pub(crate) fn new(_len: Option<usize>) -> Self {
        Self {
            fields: BTreeMap::new(),
            key: None,
        }
    }
}

impl serde::ser::SerializeMap for FirestoreMapValueSerializer {
    type Ok = Value;

    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if let Value {
            value_type: Some(ValueType::StringValue(key_string)),
        } = key.serialize(Serializer::new())?
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

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if let Some(k) = self.key.take() {
            let v = value.serialize(Serializer::new())?;
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

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
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

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        serde::ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeMap::end(self)
    }
}
