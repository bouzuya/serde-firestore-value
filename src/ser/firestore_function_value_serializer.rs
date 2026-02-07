#[cfg(feature = "btree-map")]
use std::collections::BTreeMap;
#[cfg(feature = "hash-map")]
use std::collections::HashMap;

use super::firestore_value_serializer::FirestoreValueSerializer;

use crate::google::firestore::v1::Value;
use crate::{Error, error::ErrorCode, value_ext::ValueExt};

#[doc(hidden)]
pub struct FirestoreFunctionValueSerializer {
    name: Option<String>,
    args: Option<Vec<Value>>,
    #[cfg(feature = "btree-map")]
    options: Option<BTreeMap<String, Value>>,
    #[cfg(feature = "hash-map")]
    options: Option<HashMap<String, Value>>,
}

impl FirestoreFunctionValueSerializer {
    pub(crate) fn new() -> Self {
        Self {
            name: None,
            args: None,
            options: None,
        }
    }
}

impl serde::ser::SerializeStruct for FirestoreFunctionValueSerializer {
    type Ok = Value;

    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if key == "name" {
            let value = value.serialize(FirestoreValueSerializer::new())?;
            let value = value.as_string()?;
            self.name = Some(value.clone());
        } else if key == "args" {
            let value = value.serialize(FirestoreValueSerializer::new())?;
            let values = value.as_values()?;
            self.args = Some(values.clone());
        } else if key == "options" {
            let value = value.serialize(FirestoreValueSerializer::new())?;
            let fields = value.as_fields()?;
            self.options = Some(fields.clone());
        } else {
            return Err(Self::Error::from(ErrorCode::Custom(format!(
                "unexpected field: {}",
                key
            ))));
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let function = match (self.name, self.args, self.options) {
            (Some(name), Some(args), Some(options)) => Ok(crate::google::firestore::v1::Function {
                name,
                args,
                options,
            }),
            _ => Err(Self::Error::from(ErrorCode::Custom(
                "missing required fields for Function".to_string(),
            ))),
        }?;
        Ok(Value::from_function(function))
    }
}
