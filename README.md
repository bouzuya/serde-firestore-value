# serde-firestore-value

## TODOs

- ☐ chrono feature
- ☐ time feature
- ☐ crates.io

## NOTE

### serializer mapping table

| [serde data model]         | [firestore Value]                   |
|----------------------------|-------------------------------------|
| bool                       | booleanValue                        |
| i8                         | integerValue                        |
| i16                        | integerValue                        |
| i32                        | integerValue                        |
| i64                        | integerValue                        |
| i128                       | (not supported)                     |
| u8                         | integerValue                        |
| u16                        | integerValue                        |
| u32                        | integerValue                        |
| u64                        | (not supported)                     |
| u128                       | (not supported)                     |
| f32                        | doubleValue                         |
| f64                        | doubleValue                         |
| char                       | stringValue                         |
| string                     | stringValue                         |
| byte array                 | bytesValue                          |
| option                     | nullValue or (value)                |
| unit                       | nullValue                           |
| unit_struct                | nullValue                           |
| unit_variant               | stringValue                         |
| newtype_struct             | (value)                             |
| newtype_struct (reference) | referenceValue                      |
| newtype_variant            | mapValue (`{ (name): (value) }`)    |
| seq                        | arrayValue                          |
| tuple                      | arrayValue                          |
| tuple_struct               | arrayValue                          |
| tuple_variant              | mapValue (`{ (name): arrayValue }`) |
| map                        | mapValue (`{ (key): (value) }`)     |
| struct                     | mapValue (`{ (field): (value) }`)   |
| struct (lat_lng)           | geoPointValue                       |
| struct (timestamp)         | timestampValue                      |
| struct_variant             | mapValue (`{ (name): mapValue }`)   |

### deserializer mapping table (no type hint)

| [firestore Value]  | [serde data model]                            |
|--------------------|-----------------------------------------------|
| nullValue          | unit                                          |
| booleanValue       | bool                                          |
| integerValue       | i64                                           |
| doubleValue        | f64                                           |
| timestampValue     | map (`{ "seconds": i64, "nanos": i64 }`)      |
| stringValue        | string                                        |
| bytesValue         | bytes                                         |
| referenceValue     | string                                        |
| geoPointValue      | map (`{ "latitude": f64, "longitude": f64 }`) |
| arrayValue         | seq                                           |
| mapValue           | map                                           |

[serde data model]: https://serde.rs/data-model.html
[firestore Value]: https://firebase.google.com/docs/firestore/reference/rest/v1/Value

### firestore value types

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
