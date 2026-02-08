#[cfg(feature = "btree-map")]
use std::collections::BTreeMap;
#[cfg(feature = "hash-map")]
use std::collections::HashMap;

use crate::google::firestore::v1::Value;

/// Pipeline
///
/// `pipelineValue` inner type.
///
/// <https://firebase.google.com/docs/firestore/reference/rest/Shared.Types/ArrayValue#Value>
#[derive(Clone, Debug, PartialEq)]
pub struct Pipeline {
    /// Required. Ordered list of stages to evaluate.
    pub stages: Vec<Stage>,
}

impl Pipeline {
    pub(crate) const FIELDS: &'static [&'static str] = &["stages"];
    pub(crate) const NAME: &'static str = "$__serde-firestore-value_private_pipeline";
}

impl<'de> serde::Deserialize<'de> for Pipeline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct(Self::NAME, &Self::FIELDS, PipelineVisitor)
    }
}

impl serde::Serialize for Pipeline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct(Self::NAME, Self::FIELDS.len())?;
        state.serialize_field("stages", &self.stages)?;
        state.end()
    }
}

/// Stage
///
/// A single operation within a pipeline.
///
/// <https://firebase.google.com/docs/firestore/reference/rest/Shared.Types/ArrayValue#Value>
#[derive(Clone, Debug, PartialEq)]
pub struct Stage {
    /// Required. The name of the stage to evaluate.
    pub name: String,
    /// Optional. Ordered list of arguments the given stage expects.
    pub args: Vec<Value>,
    /// Optional. Optional named arguments that certain functions may support.
    #[cfg(feature = "btree-map")]
    pub options: BTreeMap<String, Value>,
    /// Optional. Optional named arguments that certain functions may support.
    #[cfg(feature = "hash-map")]
    pub options: HashMap<String, Value>,
}

impl Stage {
    pub(crate) const FIELDS: &'static [&'static str] = &["name", "args", "options"];
    pub(crate) const NAME: &'static str = "$__serde-firestore-value_private_pipeline_stage";
}

impl serde::Serialize for Stage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct(Self::NAME, Self::FIELDS.len())?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field(
            "args",
            &self
                .args
                .iter()
                .map(crate::typ::private::ValueWrapper)
                .collect::<Vec<_>>(),
        )?;
        state.serialize_field(
            "options",
            &self
                .options
                .iter()
                .map(|(k, v)| (k.as_str(), crate::typ::private::ValueWrapper(v)))
                .collect::<HashMap<_, _>>(),
        )?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for Stage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct(Self::NAME, &Self::FIELDS, StageVisitor)
    }
}

struct StageVisitor;

impl<'de> serde::de::Visitor<'de> for StageVisitor {
    type Value = Stage;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Stage struct")
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
                    args = Some(map.next_value_seed(crate::typ::private::ValueVecSeed)?);
                }
                "options" => {
                    if options.is_some() {
                        return Err(serde::de::Error::duplicate_field("options"));
                    }
                    options = Some(map.next_value_seed(crate::typ::private::ValueMapSeed)?);
                }
                _ => {
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
            }
        }

        let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
        let args = args.ok_or_else(|| serde::de::Error::missing_field("args"))?;
        let options = options.ok_or_else(|| serde::de::Error::missing_field("options"))?;

        Ok(Stage {
            name,
            args,
            options,
        })
    }
}

struct PipelineVisitor;

impl<'de> serde::de::Visitor<'de> for PipelineVisitor {
    type Value = Pipeline;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Pipeline struct")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut stages: Option<Vec<Stage>> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "stages" => {
                    if stages.is_some() {
                        return Err(serde::de::Error::duplicate_field("stages"));
                    }
                    stages = Some(map.next_value_seed(StageVecSeed)?);
                }
                _ => {
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
            }
        }

        let stages = stages.ok_or_else(|| serde::de::Error::missing_field("stages"))?;

        Ok(Pipeline { stages })
    }
}

struct StageVecSeed;

impl<'de> serde::de::DeserializeSeed<'de> for StageVecSeed {
    type Value = Vec<Stage>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(StageVecVisitor)
    }
}

struct StageVecVisitor;

impl<'de> serde::de::Visitor<'de> for StageVecVisitor {
    type Value = Vec<Stage>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence of Stages")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut stages = Vec::new();
        while let Some(stage) = seq.next_element_seed(StageSeed)? {
            stages.push(stage);
        }
        Ok(stages)
    }
}

struct StageSeed;

impl<'de> serde::de::DeserializeSeed<'de> for StageSeed {
    type Value = Stage;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde::Deserialize::deserialize(deserializer)
    }
}
