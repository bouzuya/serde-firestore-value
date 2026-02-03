#[derive(Clone, Copy, Debug)]
pub(super) enum ValueTypeName {
    Null,
    Boolean,
    Integer,
    Double,
    Timestamp,
    String,
    Bytes,
    Reference,
    GeoPoint,
    Array,
    Map,
    FieldReference,
    Function,
    Pipeline,
}

impl ValueTypeName {
    pub(super) fn as_str(&self) -> &'static str {
        match self {
            ValueTypeName::Null => "null value",
            ValueTypeName::Boolean => "boolean value",
            ValueTypeName::Integer => "integer value",
            ValueTypeName::Double => "double value",
            ValueTypeName::Timestamp => "timestamp value",
            ValueTypeName::String => "string value",
            ValueTypeName::Bytes => "bytes value",
            ValueTypeName::Reference => "reference value",
            ValueTypeName::GeoPoint => "geo point value",
            ValueTypeName::Array => "array value",
            ValueTypeName::Map => "map value",
            ValueTypeName::FieldReference => "field reference value",
            ValueTypeName::Function => "function value",
            ValueTypeName::Pipeline => "pipeline value",
        }
    }
}
