/*
    This file shows how to use the newtype idiom in order
    to write a serde converter for unsupported types.

    It implements fmt::Debug, Serialize, and Deserialize for the
    given newtype.

    Based on the documention: https://serde.rs/
*/

use std::fmt;
use cgmath::Vector2;
use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde::de::{Visitor, Deserialize, Deserializer, SeqAccess, Error};

pub struct Vec2(Vector2<f32>);

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2(Vector2::new(x, y))
    }
}

// -------------- DEBUG ------------------- //

impl fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec2 [x: {}, y:{}]", self.0.x, self.0.y)
    }
}

// ------------- SERIALIZE ------------------- //

impl Serialize for Vec2
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.0.x)?;
        seq.serialize_element(&self.0.y)?;
        seq.end()
    }
}

// ----------- DESERIALIZE ------------------- //

struct Vec2Visitor;

impl<'de> Visitor<'de> for Vec2Visitor {
    type Value = Vec2;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Vec2")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>, {

        let x: f32 = seq.next_element()?
                .ok_or_else(|| Error::invalid_length(0, &self))?;

        let y: f32 = seq.next_element()?
            .ok_or_else(|| Error::invalid_length(1, &self))?;
        Ok(Vec2(Vector2::new(x, y)))
    }
}

impl<'de> Deserialize<'de> for Vec2 {
    fn deserialize<D>(deserializer: D) -> Result<Vec2, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(Vec2Visitor)
    }
}