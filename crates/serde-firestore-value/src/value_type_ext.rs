use google::firestore::v1::value::ValueType;

use super::value_type_name::ValueTypeName;

pub(super) trait ValueTypeExt {
    fn name(&self) -> ValueTypeName;
}

impl ValueTypeExt for ValueType {
    fn name(&self) -> ValueTypeName {
        match self {
            ValueType::NullValue(_) => ValueTypeName::Null,
            ValueType::BooleanValue(_) => ValueTypeName::Boolean,
            ValueType::IntegerValue(_) => ValueTypeName::Integer,
            ValueType::DoubleValue(_) => ValueTypeName::Double,
            ValueType::TimestampValue(_) => ValueTypeName::Timestamp,
            ValueType::StringValue(_) => ValueTypeName::String,
            ValueType::BytesValue(_) => ValueTypeName::Bytes,
            ValueType::ReferenceValue(_) => ValueTypeName::Reference,
            ValueType::GeoPointValue(_) => ValueTypeName::GeoPoint,
            ValueType::ArrayValue(_) => ValueTypeName::Array,
            ValueType::MapValue(_) => ValueTypeName::Map,
        }
    }
}
