//! Modules specified in `#[serde(with = "...")]` to support special values
//! such as GeoPoint, Reference, and Timestamp.
//!
//! # Serialize and Deserialize GeoPoint, Reference, and Timestamp
//!
//! ```rust
//! # fn main() -> anyhow::Result<()> {
//! use serde_firestore_value::google::{
//!     firestore::v1::{value::ValueType, MapValue, Value},
//!     r#type::LatLng,
//! };
//! use serde_firestore_value::{
//!     from_value, to_value,
//!     with::{lat_lng, string_as_reference, timestamp},
//! };
//!
//! #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
//! struct T {
//!     #[serde(with = "lat_lng")]
//!     lat_lng: LatLng,
//!     #[serde(with = "string_as_reference")]
//!     reference: String,
//!     #[serde(with = "timestamp")]
//!     timestamp: prost_types::Timestamp,
//! }
//! let t = T {
//!     lat_lng: LatLng {
//!         latitude: 1_f64,
//!         longitude: 2_f64,
//!     },
//!     reference: "projects/p/databases/d/documents/c".to_string(),
//!     timestamp: prost_types::Timestamp {
//!         seconds: 3_i64,
//!         nanos: 4_i32,
//!     },
//! };
//! let v = Value {
//!     value_type: Some(ValueType::MapValue(MapValue {
//!         fields: std::collections::HashMap::from([
//!             (
//!                 "lat_lng".to_string(),
//!                 Value {
//!                     value_type: Some(ValueType::GeoPointValue(LatLng { latitude: 1_f64, longitude: 2_f64 })),
//!                 },
//!             ),
//!             (
//!                 "reference".to_string(),
//!                 Value {
//!                     value_type: Some(ValueType::ReferenceValue(
//!                         "projects/p/databases/d/documents/c".to_string(),
//!                     )),
//!                 },
//!             ),
//!             (
//!                 "timestamp".to_string(),
//!                 Value {
//!                     value_type: Some(ValueType::TimestampValue(prost_types::Timestamp { seconds: 3_i64, nanos: 4_i32 })),
//!                 },
//!             ),
//!         ]),
//!     })),
//! };
//!
//! let s = to_value(&t)?;
//! let d = from_value::<'_, T>(&s)?;
//! assert_eq!(s, v);
//! assert_eq!(d, t);
//! #     Ok(())
//! # }
//! ```

#[cfg(feature = "chrono")]
pub mod chrono_date_time_as_timestamp;
pub mod lat_lng;
#[cfg(feature = "chrono")]
pub mod option_chrono_date_time_as_timestamp;
pub mod option_lat_lng;
pub mod option_string_as_reference;
#[cfg(feature = "time")]
pub mod option_time_offset_date_time_as_timestamp;
pub mod option_timestamp;
pub mod string_as_reference;
#[cfg(feature = "time")]
pub mod time_offset_date_time_as_timestamp;
pub mod timestamp;
pub mod vec_string_as_reference;
