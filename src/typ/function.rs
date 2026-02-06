#[cfg(feature = "btree-map")]
use std::collections::BTreeMap;
#[cfg(feature = "hash-map")]
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
    #[cfg(feature = "btree-map")]
    pub options: BTreeMap<String, Value>,
    /// Optional. Optional named arguments that certain functions may support.
    #[cfg(feature = "hash-map")]
    pub options: HashMap<String, Value>,
}

impl serde::Serialize for Function {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct(Self::NAME, 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("args", &ValueVecWrapper(&self.args))?;
        state.serialize_field("options", &ValueMapWrapper(&self.options))?;
        state.end()
    }
}

impl Function {
    pub(crate) const NAME: &'static str = "$__serde-firestore-value_private_function";
}

struct ValueWrapper<'a>(&'a Value);

impl serde::Serialize for ValueWrapper<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::{SerializeMap, SerializeSeq};

        use crate::google::firestore::v1::value::ValueType;

        match &self.0.value_type {
            None => serializer.serialize_none(),
            Some(ValueType::NullValue(_)) => serializer.serialize_none(),
            Some(ValueType::BooleanValue(v)) => serializer.serialize_bool(*v),
            Some(ValueType::IntegerValue(v)) => serializer.serialize_i64(*v),
            Some(ValueType::DoubleValue(v)) => serializer.serialize_f64(*v),
            Some(ValueType::TimestampValue(v)) => {
                use serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct(crate::Timestamp::NAME, 2)?;
                state.serialize_field("seconds", &v.seconds)?;
                state.serialize_field("nanos", &v.nanos)?;
                state.end()
            }
            Some(ValueType::StringValue(v)) => serializer.serialize_str(v),
            Some(ValueType::BytesValue(v)) => serializer.serialize_bytes(v),
            Some(ValueType::ReferenceValue(v)) => {
                serializer.serialize_newtype_struct(crate::Reference::NAME, v)
            }
            Some(ValueType::GeoPointValue(v)) => {
                use serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct(crate::LatLng::NAME, 2)?;
                state.serialize_field("latitude", &v.latitude)?;
                state.serialize_field("longitude", &v.longitude)?;
                state.end()
            }
            Some(ValueType::ArrayValue(v)) => {
                let mut seq = serializer.serialize_seq(Some(v.values.len()))?;
                for value in &v.values {
                    seq.serialize_element(&ValueWrapper(value))?;
                }
                seq.end()
            }
            Some(ValueType::MapValue(v)) => {
                let mut map = serializer.serialize_map(Some(v.fields.len()))?;
                for (k, value) in &v.fields {
                    map.serialize_entry(k, &ValueWrapper(value))?;
                }
                map.end()
            }
            Some(ValueType::FieldReferenceValue(v)) => {
                serializer.serialize_newtype_struct(crate::FieldReference::NAME, v)
            }
            Some(ValueType::FunctionValue(v)) => {
                use serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct(Function::NAME, 3)?;
                state.serialize_field("name", &v.name)?;
                let args: Vec<Value> = v.args.clone();
                state.serialize_field("args", &ValueVecWrapper(&args))?;
                #[cfg(feature = "btree-map")]
                let options: BTreeMap<String, Value> = v.options.clone();
                #[cfg(feature = "hash-map")]
                let options: HashMap<String, Value> = v.options.clone();
                state.serialize_field("options", &ValueMapWrapper(&options))?;
                state.end()
            }
            Some(ValueType::PipelineValue(_)) => {
                // PipelineValue is not supported in serialization
                Err(serde::ser::Error::custom("PipelineValue is not supported"))
            }
        }
    }
}

struct ValueVecWrapper<'a>(&'a Vec<Value>);

impl serde::Serialize for ValueVecWrapper<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for value in self.0 {
            seq.serialize_element(&ValueWrapper(value))?;
        }
        seq.end()
    }
}

#[cfg(feature = "btree-map")]
struct ValueMapWrapper<'a>(&'a BTreeMap<String, Value>);
#[cfg(feature = "hash-map")]
struct ValueMapWrapper<'a>(&'a HashMap<String, Value>);

impl serde::Serialize for ValueMapWrapper<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in self.0 {
            map.serialize_entry(k, &ValueWrapper(v))?;
        }
        map.end()
    }
}
