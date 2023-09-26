use google_api_proto::google::{
    firestore::v1::{value::ValueType, ArrayValue, MapValue, Value},
    r#type::LatLng,
};
use prost_types::Timestamp;

use crate::{error::ErrorCode, value_type_name::ValueTypeName, Error};

pub(super) trait ValueExt {
    fn as_null(&self) -> Result<(), Error>;
    fn as_boolean(&self) -> Result<bool, Error>;
    fn as_integer(&self) -> Result<i64, Error>;
    fn as_double(&self) -> Result<f64, Error>;
    fn as_timestamp(&self) -> Result<&Timestamp, Error>;
    fn as_string(&self) -> Result<&String, Error>;
    fn as_bytes(&self) -> Result<&[u8], Error>;
    fn as_reference_value_as_string(&self) -> Result<&String, Error>;
    fn as_lat_lng(&self) -> Result<&LatLng, Error>;
    fn as_array(&self) -> Result<&ArrayValue, Error>;
    fn as_map(&self) -> Result<&MapValue, Error>;
    fn as_variant_value(&self, variants: &'static [&'static str]) -> Result<&Value, Error>;
    fn value_type(&self) -> Result<&ValueType, Error>;
}

impl ValueExt for Value {
    fn as_null(&self) -> Result<(), Error> {
        match self.value_type()? {
            ValueType::NullValue(_) => Ok(()),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Null)),
        }
    }

    fn as_boolean(&self) -> Result<bool, Error> {
        match self.value_type()? {
            ValueType::BooleanValue(value) => Ok(*value),
            value_type => Err(Error::invalid_value_type(
                value_type,
                ValueTypeName::Boolean,
            )),
        }
    }

    fn as_integer(&self) -> Result<i64, Error> {
        match self.value_type()? {
            ValueType::IntegerValue(value) => Ok(*value),
            value_type => Err(Error::invalid_value_type(
                value_type,
                ValueTypeName::Integer,
            )),
        }
    }

    fn as_double(&self) -> Result<f64, Error> {
        match self.value_type()? {
            ValueType::DoubleValue(value) => Ok(*value),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Double)),
        }
    }

    fn as_timestamp(&self) -> Result<&Timestamp, Error> {
        match self.value_type()? {
            ValueType::TimestampValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(
                value_type,
                ValueTypeName::Timestamp,
            )),
        }
    }

    fn as_string(&self) -> Result<&String, Error> {
        match self.value_type()? {
            ValueType::StringValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::String)),
        }
    }

    fn as_bytes(&self) -> Result<&[u8], Error> {
        match self.value_type()? {
            ValueType::BytesValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Bytes)),
        }
    }

    fn as_reference_value_as_string(&self) -> Result<&String, Error> {
        match self.value_type()? {
            ValueType::ReferenceValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(
                value_type,
                ValueTypeName::Reference,
            )),
        }
    }

    fn as_lat_lng(&self) -> Result<&LatLng, Error> {
        match self.value_type()? {
            ValueType::GeoPointValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(
                value_type,
                ValueTypeName::GeoPoint,
            )),
        }
    }

    fn as_array(&self) -> Result<&ArrayValue, Error> {
        match self.value_type()? {
            ValueType::ArrayValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Array)),
        }
    }

    fn as_map(&self) -> Result<&MapValue, Error> {
        match self.value_type()? {
            ValueType::MapValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Map)),
        }
    }

    fn as_variant_value(&self, variants: &'static [&'static str]) -> Result<&Value, Error> {
        let MapValue { fields } = self.as_map()?;
        if fields.len() != 1 {
            todo!()
        }
        let (variant, value) = fields.iter().next().expect("fields must have an entry");
        if !variants.contains(&variant.as_str()) {
            todo!()
        }
        Ok(value)
    }

    fn value_type(&self) -> Result<&ValueType, Error> {
        self.value_type
            .as_ref()
            .ok_or_else(|| Error::from(ErrorCode::ValueTypeMustBeSome))
    }
}
