#[cfg(feature = "btree-map")]
use std::collections::BTreeMap;
#[cfg(feature = "hash-map")]
use std::collections::HashMap;

#[cfg(feature = "bytes")]
use prost::bytes::Bytes;

use crate::google::{
    firestore::v1::{ArrayValue, MapValue, Value, value::ValueType},
    r#type::LatLng as GoogleApiProtoLatLng,
};
use crate::{Error, error::ErrorCode, value_type_name::ValueTypeName};

pub(crate) trait ValueExt {
    fn from_bool(value: bool) -> Self;
    fn from_bytes(value: Vec<u8>) -> Self;
    fn from_f64(value: f64) -> Self;
    fn from_fields<I, S>(fields: I) -> Self
    where
        I: IntoIterator<Item = (S, Value)>,
        S: Into<String>;
    fn from_i64(value: i64) -> Self;
    fn from_lat_lng(value: GoogleApiProtoLatLng) -> Self;
    fn from_string(value: String) -> Self;
    fn from_string_as_reference_value(value: String) -> Self;
    fn from_timestamp(timestamp: prost_types::Timestamp) -> Self;
    fn from_values(values: Vec<Value>) -> Self;
    fn null() -> Self;

    fn as_boolean(&self) -> Result<bool, Error>;
    fn as_bytes(&self) -> Result<&[u8], Error>;
    fn as_double(&self) -> Result<f64, Error>;
    fn as_field_reference_value_as_string(&self) -> Result<&String, Error>;
    #[cfg(feature = "btree-map")]
    fn as_fields(&self) -> Result<&BTreeMap<String, Value>, Error>;
    #[cfg(feature = "hash-map")]
    fn as_fields(&self) -> Result<&HashMap<String, Value>, Error>;
    fn as_integer(&self) -> Result<i64, Error>;
    fn as_lat_lng(&self) -> Result<&GoogleApiProtoLatLng, Error>;
    fn as_null(&self) -> Result<(), Error>;
    fn as_reference_value_as_string(&self) -> Result<&String, Error>;
    fn as_string(&self) -> Result<&String, Error>;
    fn as_timestamp(&self) -> Result<&prost_types::Timestamp, Error>;
    fn as_values(&self) -> Result<&Vec<Value>, Error>;
    fn as_variant_value(&self) -> Result<(&String, &Value), Error>;
    fn value_type(&self) -> Result<&ValueType, Error>;
}

impl ValueExt for Value {
    fn from_bool(value: bool) -> Self {
        Self {
            value_type: Some(ValueType::BooleanValue(value)),
        }
    }

    #[cfg(feature = "vec-u8")]
    fn from_bytes(value: Vec<u8>) -> Self {
        Self {
            value_type: Some(ValueType::BytesValue(value)),
        }
    }

    #[cfg(feature = "bytes")]
    fn from_bytes(value: Vec<u8>) -> Self {
        Self {
            value_type: Some(ValueType::BytesValue(Bytes::from(value))),
        }
    }

    fn from_f64(value: f64) -> Self {
        Self {
            value_type: Some(ValueType::DoubleValue(value)),
        }
    }

    #[cfg(feature = "btree-map")]
    fn from_fields<I, S>(fields: I) -> Self
    where
        I: IntoIterator<Item = (S, Value)>,
        S: Into<String>,
    {
        Self {
            value_type: Some(ValueType::MapValue(MapValue {
                fields: fields
                    .into_iter()
                    .map(|(s, v)| (s.into(), v))
                    .collect::<BTreeMap<String, Value>>(),
            })),
        }
    }

    #[cfg(feature = "hash-map")]
    fn from_fields<I, S>(fields: I) -> Self
    where
        I: IntoIterator<Item = (S, Value)>,
        S: Into<String>,
    {
        Self {
            value_type: Some(ValueType::MapValue(MapValue {
                fields: fields
                    .into_iter()
                    .map(|(s, v)| (s.into(), v))
                    .collect::<HashMap<String, Value>>(),
            })),
        }
    }

    fn from_i64(value: i64) -> Self {
        Self {
            value_type: Some(ValueType::IntegerValue(value)),
        }
    }

    fn from_lat_lng(value: GoogleApiProtoLatLng) -> Self {
        Self {
            value_type: Some(ValueType::GeoPointValue(value)),
        }
    }

    fn from_string(value: String) -> Self {
        Self {
            value_type: Some(ValueType::StringValue(value)),
        }
    }

    fn from_timestamp(timestamp: prost_types::Timestamp) -> Self {
        Self {
            value_type: Some(ValueType::TimestampValue(timestamp)),
        }
    }

    fn from_string_as_reference_value(value: String) -> Self {
        Self {
            value_type: Some(ValueType::ReferenceValue(value)),
        }
    }

    fn from_values(values: Vec<Value>) -> Self {
        Self {
            value_type: Some(ValueType::ArrayValue(ArrayValue { values })),
        }
    }

    fn null() -> Self {
        Self {
            value_type: Some(ValueType::NullValue(0_i32)),
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

    fn as_bytes(&self) -> Result<&[u8], Error> {
        match self.value_type()? {
            ValueType::BytesValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Bytes)),
        }
    }

    fn as_double(&self) -> Result<f64, Error> {
        match self.value_type()? {
            ValueType::DoubleValue(value) => Ok(*value),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Double)),
        }
    }

    fn as_field_reference_value_as_string(&self) -> Result<&String, Error> {
        match self.value_type()? {
            ValueType::FieldReferenceValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(
                value_type,
                ValueTypeName::FieldReference,
            )),
        }
    }

    #[cfg(feature = "btree-map")]
    fn as_fields(&self) -> Result<&BTreeMap<String, Value>, Error> {
        match self.value_type()? {
            ValueType::MapValue(MapValue { fields }) => Ok(fields),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Map)),
        }
    }

    #[cfg(feature = "hash-map")]
    fn as_fields(&self) -> Result<&HashMap<String, Value>, Error> {
        match self.value_type()? {
            ValueType::MapValue(MapValue { fields }) => Ok(fields),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Map)),
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

    fn as_lat_lng(&self) -> Result<&GoogleApiProtoLatLng, Error> {
        match self.value_type()? {
            ValueType::GeoPointValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(
                value_type,
                ValueTypeName::GeoPoint,
            )),
        }
    }

    fn as_null(&self) -> Result<(), Error> {
        match self.value_type()? {
            ValueType::NullValue(_) => Ok(()),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Null)),
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

    fn as_string(&self) -> Result<&String, Error> {
        match self.value_type()? {
            ValueType::StringValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::String)),
        }
    }

    fn as_timestamp(&self) -> Result<&prost_types::Timestamp, Error> {
        match self.value_type()? {
            ValueType::TimestampValue(value) => Ok(value),
            value_type => Err(Error::invalid_value_type(
                value_type,
                ValueTypeName::Timestamp,
            )),
        }
    }

    fn as_values(&self) -> Result<&Vec<Value>, Error> {
        match self.value_type()? {
            ValueType::ArrayValue(ArrayValue { values }) => Ok(values),
            value_type => Err(Error::invalid_value_type(value_type, ValueTypeName::Array)),
        }
    }

    fn as_variant_value(&self) -> Result<(&String, &Value), Error> {
        let fields = self.as_fields()?;
        if fields.len() != 1 {
            return Err(<Error as serde::de::Error>::invalid_length(
                fields.len(),
                &"1",
            ));
        }
        Ok(fields.iter().next().expect("fields must have an entry"))
    }

    fn value_type(&self) -> Result<&ValueType, Error> {
        self.value_type
            .as_ref()
            .ok_or_else(|| Error::from(ErrorCode::ValueTypeMustBeSome))
    }
}
