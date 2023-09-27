use std::collections::HashMap;

use google_api_proto::google::{firestore::v1::Value, r#type::LatLng};

use super::firestore_value_serializer::FirestoreValueSerializer;

use crate::{value_ext::ValueExt, Error};

pub(crate) struct FirestoreGeoPointValueSerializer {
    fields: HashMap<&'static str, f64>,
}

impl FirestoreGeoPointValueSerializer {
    const FIELDS: [&'static str; 2] = ["latitude", "longitude"];

    pub(crate) fn new() -> Self {
        Self {
            fields: HashMap::with_capacity(2),
        }
    }

    fn get(&self, field: &'static str) -> Result<f64, Error> {
        self.fields.get(field).copied().ok_or_else(|| {
            <Error as serde::ser::Error>::custom(format!("missing field `{}`", field))
        })
    }
}

impl serde::ser::SerializeStruct for FirestoreGeoPointValueSerializer {
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
        if !Self::FIELDS.contains(&key) {
            return Err(<Self::Error as serde::ser::Error>::custom(format!(
                "unknown field `{}`, expected `{}` or `{}`",
                key,
                Self::FIELDS[0],
                Self::FIELDS[1]
            )));
        }
        let value = value.serialize(FirestoreValueSerializer)?;
        let value = value.as_double()?;
        self.fields.insert(key, value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from_lat_lng(LatLng {
            latitude: self.get(Self::FIELDS[0])?,
            longitude: self.get(Self::FIELDS[1])?,
        }))
    }
}
