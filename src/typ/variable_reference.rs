/// VariableReference
///
/// `variableReferenceValue` inner type.
///
/// <https://firebase.google.com/docs/firestore/reference/rest/Shared.Types/ArrayValue#Value>
///
/// # Examples
///
/// ```rust
/// # fn test_variable_reference() -> Result<(), serde_firestore_value::Error> {
/// #     use serde_firestore_value::google::firestore::v1::{value::ValueType, Value};
/// #     use serde_firestore_value::{from_value, VariableReference};
///  let inner = "variable_name";
///  let o = VariableReference(inner.to_string());
///  let v = Value {
///      value_type: Some(ValueType::VariableReferenceValue(inner.to_string())),
///  };
///  let d = from_value::<'_, VariableReference>(&v)?;
///  assert_eq!(d, o);
/// #     Ok(())
/// # }
/// ```
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename = "$__serde-firestore-value_private_variable_reference")]
pub struct VariableReference(pub String);

impl VariableReference {
    pub(crate) const NAME: &'static str = "$__serde-firestore-value_private_variable_reference";
}

impl From<VariableReference> for String {
    fn from(VariableReference(s): VariableReference) -> Self {
        s
    }
}

impl From<String> for VariableReference {
    fn from(s: String) -> Self {
        Self(s)
    }
}
