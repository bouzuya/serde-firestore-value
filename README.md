# serde-firestore-value

## TODOs

- ☐ chrono feature
- ☐ time feature
- ☐ crates.io

## NOTE

```rust
use ::prost::alloc::string::String;
use ::prost::alloc::vec::Vec;
use ::std::collections::HashMap;
use ::core::option::Option;
use ::prost_types::Timestamp;
use /* ... */::LatLng;

struct Value {
    value_type: Option<ValueType>,
}

enum ValueType {
    NullValue(i32),
    BooleanValue(bool),
    IntegerValue(i64),
    DoubleValue(f64),
    TimestampValue(Timestamp),
    StringValue(String),
    BytesValue(Vec<u8>),
    ReferenceValue(String),
    GeoPointValue(LatLng),
    ArrayValue(ArrayValue),
    MapValue(MapValue),
}

struct ArrayValue {
    values: Vec<Value>,
}

struct MapValue {
    fields: HashMap<String, Value>,
}

struct LatLng {
    latitude: f64,
    longitude: f64,
}
```
