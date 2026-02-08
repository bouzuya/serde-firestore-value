use super::firestore_stage_value_serializer::FirestoreStageValueSerializer;

use crate::google::firestore::v1::Value;
use crate::{Error, error::ErrorCode, value_ext::ValueExt};

#[doc(hidden)]
pub struct FirestorePipelineValueSerializer {
    stages: Option<Vec<crate::google::firestore::v1::pipeline::Stage>>,
}

impl FirestorePipelineValueSerializer {
    pub(crate) fn new() -> Self {
        Self { stages: None }
    }
}

impl serde::ser::SerializeStruct for FirestorePipelineValueSerializer {
    type Ok = Value;

    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if key == "stages" {
            // Serialize the stages field as a sequence of Stage
            let value = value.serialize(StagesSerializer)?;
            self.stages = Some(value);
        } else {
            return Err(Self::Error::from(ErrorCode::Custom(format!(
                "unexpected field: {}",
                key
            ))));
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let pipeline = match self.stages {
            Some(stages) => Ok(crate::google::firestore::v1::Pipeline { stages }),
            None => Err(Self::Error::from(ErrorCode::Custom(
                "missing required fields for Pipeline".to_string(),
            ))),
        }?;
        Ok(Value::from_pipeline(pipeline))
    }
}

/// A custom serializer for serializing Vec<Stage> into Vec<pipeline::Stage>
struct StagesSerializer;

impl serde::Serializer for StagesSerializer {
    type Ok = Vec<crate::google::firestore::v1::pipeline::Stage>;
    type Error = Error;
    type SerializeSeq = StagesSeqSerializer;
    type SerializeTuple = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(StagesSeqSerializer {
            stages: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::from(ErrorCode::Custom("expected seq".to_string())))
    }
}

struct StagesSeqSerializer {
    stages: Vec<crate::google::firestore::v1::pipeline::Stage>,
}

impl serde::ser::SerializeSeq for StagesSeqSerializer {
    type Ok = Vec<crate::google::firestore::v1::pipeline::Stage>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let stage = value.serialize(StageSerializer)?;
        self.stages.push(stage);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.stages)
    }
}

/// A custom serializer for serializing a single Stage into pipeline::Stage
struct StageSerializer;

impl serde::Serializer for StageSerializer {
    type Ok = crate::google::firestore::v1::pipeline::Stage;
    type Error = Error;
    type SerializeSeq = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = FirestoreStageValueSerializer;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(FirestoreStageValueSerializer::new())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::from(ErrorCode::Custom(
            "expected struct".to_string(),
        )))
    }
}
