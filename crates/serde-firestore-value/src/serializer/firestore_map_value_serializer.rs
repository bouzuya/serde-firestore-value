use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, MapValue, Value};

use crate::{serializer::Error, value_ext::ValueExt};

use super::{error::ErrorCode, firestore_value_serializer::FirestoreValueSerializer};

pub(crate) struct FirestoreMapValueSerializer {
    key: Option<String>,
    name: Option<&'static str>,
    output: MapValue,
}

impl FirestoreMapValueSerializer {
    pub(crate) fn new(name: Option<&'static str>, len: Option<usize>) -> Self {
        Self {
            key: None,
            name,
            output: MapValue {
                fields: HashMap::with_capacity(len.unwrap_or(0)),
            },
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
            self.output.fields.insert(k, v);
            Ok(())
        } else {
            unreachable!()
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(match self.name {
            Some(name) => Value::from_fields({
                let mut fields = HashMap::new();
                fields.insert(name.to_string(), Value::from_map_value(self.output));
                fields
            }),
            None => Value::from_map_value(self.output),
        })
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
