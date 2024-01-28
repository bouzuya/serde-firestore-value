//! A [serde] (de)serializer using [Firestore Value] as its data format.
//!
//! ```rust
//! use google_api_proto::google::firestore::v1::{value::ValueType, Value};
//! use serde_firestore_value::{from_value, to_value};
//!
//! fn main() -> anyhow::Result<()> {
//!     let t = 1_i64; // T: Serialize
//!
//!     let s = to_value(&t)?;
//!     assert_eq!(s, Value {
//!         value_type: Some(ValueType::IntegerValue(1_i64))
//!     });
//!
//!     let d = from_value::<'_, i64>(&s)?;
//!     assert_eq!(d, t);
//!
//!     Ok(())
//! }
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

pub use self::de::{from_value, Deserializer};
pub use self::error::{Error, Result};
pub use self::ser::{to_value, Serializer};
pub use self::typ::{LatLng, Reference, Timestamp};
