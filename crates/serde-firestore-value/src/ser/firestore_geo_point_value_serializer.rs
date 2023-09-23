use std::collections::HashMap;

use google::{
    firestore::v1::{value::ValueType, Value},
    r#type::LatLng,
};

use super::{error::ErrorCode, firestore_value_serializer::FirestoreValueSerializer};

use crate::{ser::Error, value_ext::ValueExt};

pub(crate) struct FirestoreGeoPointValueSerializer {
    latitude: Option<f64>,
    longitude: Option<f64>,
}

impl FirestoreGeoPointValueSerializer {
    pub(crate) fn new() -> Self {
        Self {
            latitude: None,
            longitude: None,
        }
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
        if key == "latitude" {
            let value = value.serialize(FirestoreValueSerializer)?;
            let value = match value.value_type.as_ref() {
                None => todo!(),
                Some(ValueType::DoubleValue(value)) => Ok(*value),
                Some(_) => Err(Self::Error::from(ErrorCode::Custom("TODO".to_string()))),
            }?;
            self.latitude = Some(value);
        } else if key == "longitude" {
            let value = value.serialize(FirestoreValueSerializer)?;
            let value = match value.value_type.as_ref() {
                None => todo!(),
                Some(ValueType::DoubleValue(value)) => Ok(*value),
                Some(_) => Err(Self::Error::from(ErrorCode::Custom("TODO".to_string()))),
            }?;
            self.longitude = Some(value);
        } else {
            // TODO: invalid lat_lng
            todo!()
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let value = match (self.latitude, self.longitude) {
            (None, None) | (None, Some(_)) | (Some(_), None) => {
                Err(Self::Error::from(ErrorCode::Custom("TODO".to_string())))
            }
            (Some(latitude), Some(longitude)) => Ok(LatLng {
                latitude,
                longitude,
            }),
        }?;
        let value = Value::from_lat_lng(value);
        Ok(match None::<&'static str> {
            Some(name) => Value::from_fields({
                let mut fields = HashMap::new();
                fields.insert(name.to_string(), value);
                fields
            }),
            None => value,
        })
    }
}
