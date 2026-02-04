/// FieldReference
///
/// `fieldReferenceValue` inner type.
///
/// <https://firebase.google.com/docs/firestore/reference/rest/Shared.Types/ArrayValue#Value>
///
/// # Examples
///
/// ```rust
/// # fn test_field_reference() -> Result<(), serde_firestore_value::Error> {
/// #     use serde_firestore_value::google::firestore::v1::{value::ValueType, Value};
/// #     use serde_firestore_value::{from_value, FieldReference};
///  let inner = "field_name";
///  let o = FieldReference(inner.to_string());
///  let v = Value {
///      value_type: Some(ValueType::FieldReferenceValue(inner.to_string())),
///  };
///  let d = from_value::<'_, FieldReference>(&v)?;
///  assert_eq!(d, o);
/// #     Ok(())
/// # }
/// ```
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename = "$__serde-firestore-value_private_field_reference")]
pub struct FieldReference(pub String);

impl FieldReference {
    pub(crate) const NAME: &'static str = "$__serde-firestore-value_private_field_reference";
}

impl From<FieldReference> for String {
    fn from(FieldReference(s): FieldReference) -> Self {
        s
    }
}

impl From<String> for FieldReference {
    fn from(s: String) -> Self {
        Self(s)
    }
}
