use std::collections::HashMap;

use google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};

use crate::{to_value, Error};

pub(crate) trait SetMapValue {
    fn set_map_value(&mut self, value: MapValue);
}

pub(crate) struct FirestoreNamedArrayValueSerializer<'a, S: SetMapValue> {
    name: &'static str,
    output: ArrayValue,
    parent: &'a mut S,
}

impl<'a, S: SetMapValue> FirestoreNamedArrayValueSerializer<'a, S> {
    pub(crate) fn new(parent: &'a mut S, name: &'static str, len: usize) -> Self {
        Self {
            name,
            output: ArrayValue {
                values: Vec::with_capacity(len),
            },
            parent,
        }
    }
}

impl<'a, S: SetMapValue> serde::ser::SerializeTupleVariant
    for FirestoreNamedArrayValueSerializer<'a, S>
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
        self.parent.set_map_value(MapValue {
            fields: {
                let mut map = HashMap::new();
                map.insert(
                    self.name.to_string(),
                    Value {
                        value_type: Some(ValueType::ArrayValue(self.output)),
                    },
                );
                map
            },
        });
        Ok(self.parent)
    }
}
