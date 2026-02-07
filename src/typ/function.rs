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

impl<'de> serde::Deserialize<'de> for Function {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct(Self::NAME, &Self::FIELDS, FunctionVisitor)
    }
}

impl serde::Serialize for Function {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct(Self::NAME, 3)?;
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
                .collect::<HashMap<_, _>>(),
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

struct ValueVecSeed;

impl<'de> serde::de::DeserializeSeed<'de> for ValueVecSeed {
    type Value = Vec<Value>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(ValueVecVisitor)
    }
}

struct ValueVecVisitor;

impl<'de> serde::de::Visitor<'de> for ValueVecVisitor {
    type Value = Vec<Value>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence of Values")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = seq.next_element_seed(ValueSeed)? {
            values.push(value);
        }
        Ok(values)
    }
}

struct ValueMapSeed;

#[cfg(feature = "btree-map")]
impl<'de> serde::de::DeserializeSeed<'de> for ValueMapSeed {
    type Value = BTreeMap<String, Value>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ValueMapVisitor)
    }
}

#[cfg(feature = "hash-map")]
impl<'de> serde::de::DeserializeSeed<'de> for ValueMapSeed {
    type Value = HashMap<String, Value>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ValueMapVisitor)
    }
}

struct ValueMapVisitor;

#[cfg(feature = "btree-map")]
impl<'de> serde::de::Visitor<'de> for ValueMapVisitor {
    type Value = BTreeMap<String, Value>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map of String to Value")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut values = BTreeMap::new();
        while let Some(key) = map.next_key::<String>()? {
            let value = map.next_value_seed(ValueSeed)?;
            values.insert(key, value);
        }
        Ok(values)
    }
}

#[cfg(feature = "hash-map")]
impl<'de> serde::de::Visitor<'de> for ValueMapVisitor {
    type Value = HashMap<String, Value>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map of String to Value")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut values = HashMap::new();
        while let Some(key) = map.next_key::<String>()? {
            let value = map.next_value_seed(ValueSeed)?;
            values.insert(key, value);
        }
        Ok(values)
    }
}

struct ValueSeed;

impl<'de> serde::de::DeserializeSeed<'de> for ValueSeed {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

struct ValueVisitor;

impl<'de> serde::de::Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Firestore Value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use crate::google::firestore::v1::value::ValueType;
        Ok(Value {
            value_type: Some(ValueType::BooleanValue(v)),
        })
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use crate::google::firestore::v1::value::ValueType;
        Ok(Value {
            value_type: Some(ValueType::IntegerValue(v)),
        })
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use crate::google::firestore::v1::value::ValueType;
        Ok(Value {
            value_type: Some(ValueType::IntegerValue(v as i64)),
        })
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use crate::google::firestore::v1::value::ValueType;
        Ok(Value {
            value_type: Some(ValueType::DoubleValue(v)),
        })
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use crate::google::firestore::v1::value::ValueType;
        Ok(Value {
            value_type: Some(ValueType::StringValue(v.to_string())),
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use crate::google::firestore::v1::value::ValueType;
        Ok(Value {
            value_type: Some(ValueType::StringValue(v)),
        })
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use crate::value_ext::ValueExt;
        Ok(Value::from_bytes(v.to_vec()))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use crate::google::firestore::v1::value::ValueType;
        Ok(Value {
            value_type: Some(ValueType::NullValue(0)),
        })
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use crate::google::firestore::v1::value::ValueType;
        Ok(Value {
            value_type: Some(ValueType::NullValue(0)),
        })
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use crate::google::firestore::v1::{ArrayValue, value::ValueType};
        let mut values = Vec::new();
        while let Some(value) = seq.next_element_seed(ValueSeed)? {
            values.push(value);
        }
        Ok(Value {
            value_type: Some(ValueType::ArrayValue(ArrayValue { values })),
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use crate::google::firestore::v1::{MapValue, value::ValueType};

        #[cfg(feature = "btree-map")]
        let mut fields = BTreeMap::new();
        #[cfg(feature = "hash-map")]
        let mut fields = HashMap::new();

        while let Some(key) = map.next_key::<String>()? {
            if fields.contains_key(&key) {
                return Err(serde::de::Error::custom(format_args!(
                    "duplicate field `{}`",
                    key
                )));
            }
            let value = map.next_value_seed(ValueSeed)?;
            fields.insert(key, value);
        }

        // FIXME: ?
        // Check for special struct types based on field names
        // Timestamp: { seconds, nanos }
        if fields.len() == 2 && fields.contains_key("seconds") && fields.contains_key("nanos") {
            let seconds = fields
                .get("seconds")
                .and_then(|v| match &v.value_type {
                    Some(ValueType::IntegerValue(n)) => Some(*n),
                    _ => None,
                })
                .ok_or_else(|| serde::de::Error::custom("invalid seconds field"))?;
            let nanos = fields
                .get("nanos")
                .and_then(|v| match &v.value_type {
                    Some(ValueType::IntegerValue(n)) => Some(*n as i32),
                    _ => None,
                })
                .ok_or_else(|| serde::de::Error::custom("invalid nanos field"))?;
            return Ok(Value {
                value_type: Some(ValueType::TimestampValue(prost_types::Timestamp {
                    seconds,
                    nanos,
                })),
            });
        }

        // FIXME: ?
        // GeoPoint: { latitude, longitude }
        if fields.len() == 2 && fields.contains_key("latitude") && fields.contains_key("longitude")
        {
            let latitude = fields
                .get("latitude")
                .and_then(|v| match &v.value_type {
                    Some(ValueType::DoubleValue(n)) => Some(*n),
                    _ => None,
                })
                .ok_or_else(|| serde::de::Error::custom("invalid latitude field"))?;
            let longitude = fields
                .get("longitude")
                .and_then(|v| match &v.value_type {
                    Some(ValueType::DoubleValue(n)) => Some(*n),
                    _ => None,
                })
                .ok_or_else(|| serde::de::Error::custom("invalid longitude field"))?;
            return Ok(Value {
                value_type: Some(ValueType::GeoPointValue(crate::google::r#type::LatLng {
                    latitude,
                    longitude,
                })),
            });
        }

        // Reference: { $__serde-firestore-value_private_reference: string }
        if fields.len() == 1 && fields.contains_key(crate::Reference::NAME) {
            let reference = fields
                .get(crate::Reference::NAME)
                .and_then(|v| match &v.value_type {
                    Some(ValueType::StringValue(s)) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| serde::de::Error::custom("invalid reference field"))?;
            return Ok(Value {
                value_type: Some(ValueType::ReferenceValue(reference)),
            });
        }

        // FieldReference: { $__serde-firestore-value_private_field_reference: string }
        if fields.len() == 1 && fields.contains_key(crate::FieldReference::NAME) {
            let field_reference = fields
                .get(crate::FieldReference::NAME)
                .and_then(|v| match &v.value_type {
                    Some(ValueType::StringValue(s)) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| serde::de::Error::custom("invalid field_reference field"))?;
            return Ok(Value {
                value_type: Some(ValueType::FieldReferenceValue(field_reference)),
            });
        }

        // FIXME: ?
        // Function: { name, args, options }
        if fields.len() == Function::FIELDS.len()
            && fields.contains_key("name")
            && fields.contains_key("args")
            && fields.contains_key("options")
        {
            let name = fields
                .get("name")
                .and_then(|v| match &v.value_type {
                    Some(ValueType::StringValue(s)) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| serde::de::Error::custom("invalid name field"))?;

            let args = fields
                .get("args")
                .and_then(|v| match &v.value_type {
                    Some(ValueType::ArrayValue(arr)) => Some(arr.values.clone()),
                    _ => None,
                })
                .ok_or_else(|| serde::de::Error::custom("invalid args field"))?;

            let options = fields
                .get("options")
                .and_then(|v| match &v.value_type {
                    Some(ValueType::MapValue(m)) => Some(m.fields.clone()),
                    _ => None,
                })
                .ok_or_else(|| serde::de::Error::custom("invalid options field"))?;

            return Ok(Value {
                value_type: Some(ValueType::FunctionValue(
                    crate::google::firestore::v1::Function {
                        name,
                        args,
                        options,
                    },
                )),
            });
        }

        Ok(Value {
            value_type: Some(ValueType::MapValue(MapValue { fields })),
        })
    }
}

impl Function {
    pub(crate) const FIELDS: &'static [&'static str] = &["name", "args", "options"];
    pub(crate) const NAME: &'static str = "$__serde-firestore-value_private_function";
}

struct ValueWrapper<'a>(&'a Value);

impl serde::Serialize for ValueWrapper<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct};

        use crate::google::firestore::v1::value::ValueType;

        match &self.0.value_type {
            None => serializer.serialize_none(),
            Some(ValueType::NullValue(_)) => serializer.serialize_none(),
            Some(ValueType::BooleanValue(v)) => serializer.serialize_bool(*v),
            Some(ValueType::IntegerValue(v)) => serializer.serialize_i64(*v),
            Some(ValueType::DoubleValue(v)) => serializer.serialize_f64(*v),
            Some(ValueType::TimestampValue(v)) => {
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
                let mut state = serializer.serialize_struct(Function::NAME, 3)?;
                state.serialize_field("name", &v.name)?;
                state.serialize_field(
                    "args",
                    &v.args.iter().map(ValueWrapper).collect::<Vec<_>>(),
                )?;
                state.serialize_field(
                    "options",
                    &v.options
                        .iter()
                        .map(|(k, v)| (k.as_str(), ValueWrapper(v)))
                        .collect::<HashMap<_, _>>(),
                )?;
                state.end()
            }
            Some(ValueType::PipelineValue(_)) => {
                // PipelineValue is not supported in serialization
                Err(serde::ser::Error::custom("PipelineValue is not supported"))
            }
        }
    }
}
