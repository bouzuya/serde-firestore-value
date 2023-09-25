use std::collections::HashMap;

use google::{
    firestore::v1::{value::ValueType, ArrayValue, MapValue, Value},
    r#type::LatLng,
};
use prost_types::Timestamp;

pub(crate) trait ValueExt {
    fn from_bool(value: bool) -> Self;
    fn from_bytes(value: Vec<u8>) -> Self;
    fn from_f64(value: f64) -> Self;
    fn from_fields(fields: HashMap<String, Value>) -> Self;
    fn from_i64(value: i64) -> Self;
    fn from_lat_lng(value: LatLng) -> Self;
    fn from_string(value: String) -> Self;
    fn from_string_as_reference_value(value: String) -> Self;
    fn from_timestamp(timestamp: Timestamp) -> Self;
    fn from_values(values: Vec<Value>) -> Self;
    fn null() -> Self;
}

impl ValueExt for Value {
    fn from_bool(value: bool) -> Self {
        Self {
            value_type: Some(ValueType::BooleanValue(value)),
        }
    }

    fn from_bytes(value: Vec<u8>) -> Self {
        Self {
            value_type: Some(ValueType::BytesValue(value)),
        }
    }

    fn from_f64(value: f64) -> Self {
        Self {
            value_type: Some(ValueType::DoubleValue(value)),
        }
    }

    fn from_fields(fields: HashMap<String, Value>) -> Self {
        Self {
            value_type: Some(ValueType::MapValue(MapValue { fields })),
        }
    }

    fn from_i64(value: i64) -> Self {
        Self {
            value_type: Some(ValueType::IntegerValue(value)),
        }
    }

    fn from_lat_lng(value: LatLng) -> Self {
        Self {
            value_type: Some(ValueType::GeoPointValue(value)),
        }
    }

    fn from_string(value: String) -> Self {
        Self {
            value_type: Some(ValueType::StringValue(value)),
        }
    }

    fn from_timestamp(timestamp: Timestamp) -> Self {
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
}
