/// Reference
///
/// `referenceValue` inner type.
///
/// <https://firebase.google.com/docs/firestore/reference/rest/Shared.Types/ArrayValue#Value>
///
/// # Examples
///
/// ```rust
/// # fn test_reference() -> Result<(), serde_firestore_value::Error> {
/// #     use google_api_proto::google::firestore::v1::{value::ValueType, Value};
/// #     use serde_firestore_value::{from_value, to_value, Reference};
///  let inner = "projects/p/databases/d/documents/c/d";
///  let o = Reference(inner.to_string());
///  let v = Value {
///      value_type: Some(ValueType::ReferenceValue(inner.to_string())),
///  };
///  let s = to_value(&o)?;
///  let d = from_value::<'_, Reference>(&s)?;
///  assert_eq!(s, v);
///  assert_eq!(d, o);
/// #     Ok(())
/// # }
/// ```
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename = "$__serde-firestore-value_private_reference")]
pub struct Reference(pub String);

impl Reference {
    pub(crate) const NAME: &'static str = "$__serde-firestore-value_private_reference";
}

impl From<Reference> for String {
    fn from(Reference(s): Reference) -> Self {
        s
    }
}

impl From<String> for Reference {
    fn from(s: String) -> Self {
        Self(s)
    }
}
