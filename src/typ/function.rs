#[cfg(feature = "btree-map")]
use std::collections::BTreeMap;
#[cfg(feature = "hash-map")]
use std::collections::HashMap;

use super::private::{ValueMapSeed, ValueVecSeed, ValueWrapper};
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
    #[cfg(feature = "btree-map")]
    pub options: BTreeMap<String, Value>,
    /// Optional. Optional named arguments that certain functions may support.
    #[cfg(feature = "hash-map")]
    pub options: HashMap<String, Value>,
}

impl Function {
    pub(crate) const FIELDS: &'static [&'static str] = &["name", "args", "options"];
    pub(crate) const NAME: &'static str = "$__serde-firestore-value_private_function";
}

impl<'de> serde::Deserialize<'de> for Function {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct(Self::NAME, Self::FIELDS, FunctionVisitor)
    }
}

impl serde::Serialize for Function {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct(Self::NAME, Self::FIELDS.len())?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field(
            "args",
            &self.args.iter().map(ValueWrapper).collect::<Vec<_>>(),
        )?;
        state.serialize_field(
            "options",
            &self
                .options
                .iter()
                .map(|(k, v)| (k.as_str(), ValueWrapper(v)))
                .collect::<std::collections::HashMap<_, _>>(),
        )?;
        state.end()
    }
}

struct FunctionVisitor;

impl<'de> serde::de::Visitor<'de> for FunctionVisitor {
    type Value = Function;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Function struct")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut name: Option<String> = None;
        let mut args: Option<Vec<Value>> = None;
        #[cfg(feature = "btree-map")]
        let mut options: Option<BTreeMap<String, Value>> = None;
        #[cfg(feature = "hash-map")]
        let mut options: Option<HashMap<String, Value>> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "name" => {
                    if name.is_some() {
                        return Err(serde::de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }
                "args" => {
                    if args.is_some() {
                        return Err(serde::de::Error::duplicate_field("args"));
                    }
                    args = Some(map.next_value_seed(ValueVecSeed)?);
                }
                "options" => {
                    if options.is_some() {
                        return Err(serde::de::Error::duplicate_field("options"));
                    }
                    options = Some(map.next_value_seed(ValueMapSeed)?);
                }
                _ => {
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
            }
        }

        let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
        let args = args.ok_or_else(|| serde::de::Error::missing_field("args"))?;
        let options = options.ok_or_else(|| serde::de::Error::missing_field("options"))?;

        Ok(Function {
            name,
            args,
            options,
        })
    }
}
