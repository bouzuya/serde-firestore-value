use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, MapValue, Value};

use crate::{firestore_named_array_value_serializer::SetMapValue, to_value, Error};

pub(crate) struct FirestoreNamedMapValueSerializer<'a, S> {
    name: &'static str,
    output: MapValue,
    parent: &'a mut S,
}

impl<'a, S: SetMapValue> FirestoreNamedMapValueSerializer<'a, S> {
    pub(crate) fn new(parent: &'a mut S, name: &'static str, len: usize) -> Self {
        Self {
            name,
            parent,
            output: MapValue {
                fields: HashMap::with_capacity(len),
            },
        }
    }
}

impl<'a, S: SetMapValue> serde::ser::SerializeStructVariant
    for FirestoreNamedMapValueSerializer<'a, S>
{
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
        self.output
            .fields
            .insert(key.to_string(), to_value(&value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.parent.set_map_value(MapValue {
            fields: {
                let mut map = HashMap::new();
                map.insert(
                    self.name.to_string(),
                    Value {
                        value_type: Some(ValueType::MapValue(self.output)),
                    },
                );
                map
            },
        });
        Ok(self.parent)
    }
}
