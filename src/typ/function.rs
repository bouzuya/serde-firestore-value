use std::collections::HashMap;

use crate::google::firestore::v1::Value;

/// Function
///
/// `functionValue` inner type.
///
/// <https://firebase.google.com/docs/firestore/reference/rest/Shared.Types/ArrayValue#Value>
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    /// Required. The name of the function to evaluate.
    pub name: String,
    /// Optional. Ordered list of arguments the given function expects.
    pub args: Vec<Value>,
    /// Optional. Optional named arguments that certain functions may support.
    pub options: HashMap<String, Value>,
}
