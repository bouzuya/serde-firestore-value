#[cfg(feature = "btree-map")]
use std::collections::BTreeMap;
#[cfg(feature = "hash-map")]
use std::collections::HashMap;

use crate::google::firestore::v1::Value;

pub(super) struct ValueVecSeed;

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

pub(super) struct ValueMapSeed;

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

pub(super) struct ValueSeed;

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

        // Check for marker field first
        let mut marker: Option<&'static str> = None;

        while let Some(key) = map.next_key::<String>()? {
            // Detect marker fields (unit value indicates marker)
            if key == crate::Timestamp::NAME {
                let _: () = map.next_value()?;
                marker = Some(crate::Timestamp::NAME);
                continue;
            }
            if key == crate::LatLng::NAME {
                let _: () = map.next_value()?;
                marker = Some(crate::LatLng::NAME);
                continue;
            }
            if key == crate::Function::NAME {
                let _: () = map.next_value()?;
                marker = Some(crate::Function::NAME);
                continue;
            }
            if key == crate::Pipeline::NAME {
                let _: () = map.next_value()?;
                marker = Some(crate::Pipeline::NAME);
                continue;
            }
            if key == super::pipeline::Stage::NAME {
                let _: () = map.next_value()?;
                marker = Some(super::pipeline::Stage::NAME);
                continue;
            }

            if fields.contains_key(&key) {
                return Err(serde::de::Error::custom(format_args!(
                    "duplicate field `{}`",
                    key
                )));
            }
            let value = map.next_value_seed(ValueSeed)?;
            fields.insert(key, value);
        }

        // Handle marker-based special types
        if marker == Some(crate::Timestamp::NAME) {
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

        if marker == Some(crate::LatLng::NAME) {
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

        if marker == Some(crate::Function::NAME) {
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

        if marker == Some(crate::Pipeline::NAME) {
            let stages_value = fields
                .get("stages")
                .ok_or_else(|| serde::de::Error::custom("missing stages field"))?;

            let stages = match &stages_value.value_type {
                Some(ValueType::ArrayValue(arr)) => arr
                    .values
                    .iter()
                    .map(|v| match &v.value_type {
                        Some(ValueType::MapValue(m)) => {
                            let name = m
                                .fields
                                .get("name")
                                .and_then(|v| match &v.value_type {
                                    Some(ValueType::StringValue(s)) => Some(s.clone()),
                                    _ => None,
                                })
                                .ok_or_else(|| {
                                    serde::de::Error::custom("invalid stage name field")
                                })?;
                            let args = m
                                .fields
                                .get("args")
                                .and_then(|v| match &v.value_type {
                                    Some(ValueType::ArrayValue(arr)) => Some(arr.values.clone()),
                                    _ => None,
                                })
                                .ok_or_else(|| {
                                    serde::de::Error::custom("invalid stage args field")
                                })?;
                            let options = m
                                .fields
                                .get("options")
                                .and_then(|v| match &v.value_type {
                                    Some(ValueType::MapValue(m)) => Some(m.fields.clone()),
                                    _ => None,
                                })
                                .ok_or_else(|| {
                                    serde::de::Error::custom("invalid stage options field")
                                })?;
                            Ok(crate::google::firestore::v1::pipeline::Stage {
                                name,
                                args,
                                options,
                            })
                        }
                        _ => Err(serde::de::Error::custom("invalid stage value")),
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                _ => return Err(serde::de::Error::custom("invalid stages field")),
            };

            return Ok(Value {
                value_type: Some(ValueType::PipelineValue(
                    crate::google::firestore::v1::Pipeline { stages },
                )),
            });
        }

        if marker == Some(super::pipeline::Stage::NAME) {
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

            // Stage is serialized as a MapValue containing its fields
            #[cfg(feature = "btree-map")]
            let mut stage_fields = BTreeMap::new();
            #[cfg(feature = "hash-map")]
            let mut stage_fields = HashMap::new();

            stage_fields.insert(
                "name".to_string(),
                Value {
                    value_type: Some(ValueType::StringValue(name)),
                },
            );
            stage_fields.insert(
                "args".to_string(),
                Value {
                    value_type: Some(ValueType::ArrayValue(
                        crate::google::firestore::v1::ArrayValue { values: args },
                    )),
                },
            );
            stage_fields.insert(
                "options".to_string(),
                Value {
                    value_type: Some(ValueType::MapValue(
                        crate::google::firestore::v1::MapValue { fields: options },
                    )),
                },
            );

            return Ok(Value {
                value_type: Some(ValueType::MapValue(MapValue {
                    fields: stage_fields,
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

        Ok(Value {
            value_type: Some(ValueType::MapValue(MapValue { fields })),
        })
    }
}

pub(super) struct ValueWrapper<'a>(pub &'a Value);

impl serde::Serialize for ValueWrapper<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct};

        use crate::google::firestore::v1::value::ValueType;
        use crate::typ::{FieldReference, Function, LatLng, Pipeline, Reference, Stage, Timestamp};

        match &self.0.value_type {
            None => serializer.serialize_none(),
            Some(ValueType::NullValue(_)) => serializer.serialize_none(),
            Some(ValueType::BooleanValue(v)) => serializer.serialize_bool(*v),
            Some(ValueType::IntegerValue(v)) => serializer.serialize_i64(*v),
            Some(ValueType::DoubleValue(v)) => serializer.serialize_f64(*v),
            Some(ValueType::TimestampValue(v)) => {
                let mut state = serializer.serialize_struct(Timestamp::NAME, 2)?;
                state.serialize_field("seconds", &v.seconds)?;
                state.serialize_field("nanos", &v.nanos)?;
                state.end()
            }
            Some(ValueType::StringValue(v)) => serializer.serialize_str(v),
            Some(ValueType::BytesValue(v)) => serializer.serialize_bytes(v),
            Some(ValueType::ReferenceValue(v)) => {
                serializer.serialize_newtype_struct(Reference::NAME, v)
            }
            Some(ValueType::GeoPointValue(v)) => {
                let mut state = serializer.serialize_struct(LatLng::NAME, 2)?;
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
                serializer.serialize_newtype_struct(FieldReference::NAME, v)
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
            Some(ValueType::PipelineValue(v)) => {
                let mut state = serializer.serialize_struct(Pipeline::NAME, 1)?;
                let stages: Vec<Stage> = v
                    .stages
                    .iter()
                    .map(|s| Stage {
                        name: s.name.clone(),
                        args: s.args.clone(),
                        options: s.options.clone(),
                    })
                    .collect();
                state.serialize_field("stages", &stages)?;
                state.end()
            }
        }
    }
}
