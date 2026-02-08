//! A [serde] (de)serializer using [Firestore Value] as its data format.
//!
//! # Examples
//!
//! ```rust
//! # fn main() -> anyhow::Result<()> {
//! use serde_firestore_value::google::firestore::v1::{value::ValueType, Value};
//! use serde_firestore_value::{from_value, to_value};
//!
//! let t = 1_i64; // T: Serialize
//!
//! let s = to_value(&t)?;
//! assert_eq!(s, Value {
//!     value_type: Some(ValueType::IntegerValue(1_i64))
//! });
//!
//! let d = from_value::<'_, i64>(&s)?;
//! assert_eq!(d, t);
//! #     Ok(())
//! # }
//! ```
//!
//! ```rust
//! # fn test() -> anyhow::Result<()> {
//! use serde_firestore_value::google::{firestore::v1::{value::ValueType, ArrayValue, MapValue, Value}, self};
//! use serde_firestore_value::{LatLng, Reference, Timestamp};
//!
//! #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
//! struct T {
//!     b: bool,
//!     i: i64,
//!     d: f64,
//!     t: Timestamp,
//!     s: String,
//!     r: Reference,
//!     g: LatLng,
//!     a: Vec<Option<i64>>,
//!     m: std::collections::HashMap<String, bool>,
//! }
//!
//! let t = T {
//!     b: true,
//!     i: 1_i64,
//!     d: 2_f64,
//!     t: Timestamp {
//!         seconds: 3_i64,
//!         nanos: 4_i32,
//!     },
//!     s: "s".to_string(),
//!     r: Reference("projects/p/databases/d/documents/n".to_string()),
//!     g: LatLng {
//!         latitude: 5_f64,
//!         longitude: 6_f64,
//!     },
//!     a: vec![Some(1), Some(2), None],
//!     m: {
//!         let mut m = std::collections::HashMap::new();
//!         m.insert("a".to_string(), false);
//!         m.insert("b".to_string(), true);
//!         m
//!     },
//! };
//! let value = Value {
//!     value_type: Some(ValueType::MapValue(MapValue {
//!         fields: {
//!             let mut fields = std::collections::HashMap::new();
//!             fields.insert(
//!                 "b".to_string(),
//!                 Value {
//!                     value_type: Some(ValueType::BooleanValue(true)),
//!                 },
//!             );
//!             fields.insert(
//!                 "i".to_string(),
//!                 Value {
//!                     value_type: Some(ValueType::IntegerValue(1)),
//!                 },
//!             );
//!             fields.insert(
//!                 "d".to_string(),
//!                 Value {
//!                     value_type: Some(ValueType::DoubleValue(2_f64)),
//!                 },
//!             );
//!             fields.insert(
//!                 "t".to_string(),
//!                 Value {
//!                     value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
//!                         seconds: 3_i64,
//!                         nanos: 4_i32,
//!                     })),
//!                 },
//!             );
//!             fields.insert(
//!                 "s".to_string(),
//!                 Value {
//!                     value_type: Some(ValueType::StringValue("s".to_string())),
//!                 },
//!             );
//!             fields.insert(
//!                 "r".to_string(),
//!                 Value {
//!                     value_type: Some(ValueType::ReferenceValue(
//!                         "projects/p/databases/d/documents/n".to_string(),
//!                     )),
//!                 },
//!             );
//!             fields.insert(
//!                 "g".to_string(),
//!                 Value {
//!                     value_type: Some(ValueType::GeoPointValue(
//!                         google::r#type::LatLng {
//!                             latitude: 5_f64,
//!                             longitude: 6_f64,
//!                         },
//!                     )),
//!                 },
//!             );
//!             fields.insert(
//!                 "a".to_string(),
//!                 Value {
//!                     value_type: Some(ValueType::ArrayValue(ArrayValue {
//!                         values: vec![
//!                             Value {
//!                                 value_type: Some(ValueType::IntegerValue(1)),
//!                             },
//!                             Value {
//!                                 value_type: Some(ValueType::IntegerValue(2)),
//!                             },
//!                             Value {
//!                                 value_type: Some(ValueType::NullValue(0)),
//!                             },
//!                         ],
//!                     })),
//!                 },
//!             );
//!             fields.insert(
//!                 "m".to_string(),
//!                 Value {
//!                     value_type: Some(ValueType::MapValue(MapValue {
//!                         fields: {
//!                             let mut fields = std::collections::HashMap::new();
//!                             fields.insert(
//!                                 "a".to_string(),
//!                                 Value {
//!                                     value_type: Some(ValueType::BooleanValue(false)),
//!                                 },
//!                             );
//!                             fields.insert(
//!                                 "b".to_string(),
//!                                 Value {
//!                                     value_type: Some(ValueType::BooleanValue(true)),
//!                                 },
//!                             );
//!                             fields
//!                         },
//!                     })),
//!                 },
//!             );
//!             fields
//!         },
//!     })),
//! };
//!
//! let serialized = serde_firestore_value::to_value(&t)?;
//! assert_eq!(serialized, value);
//!
//! let deserialized = serde_firestore_value::from_value::<T>(&serialized)?;
//! assert_eq!(deserialized, t);
//!
//! #     Ok(())
//! # }
//! ```
//!
//! [serde]: https://crates.io/crates/serde
//! [Firestore Value]: https://firebase.google.com/docs/firestore/reference/rest/v1/Value
mod de;
mod error;
mod ser;
mod serde_json;
mod typ;
mod value_ext;
mod value_type_ext;
mod value_type_name;
pub mod with;

pub use self::de::{Deserializer, from_value};
pub use self::error::{Error, Result};
pub use self::ser::{Serializer, to_value};
pub use self::typ::{FieldReference, Function, LatLng, Pipeline, Reference, Stage, Timestamp};

pub use googleapis_tonic_google_firestore_v1::google;
