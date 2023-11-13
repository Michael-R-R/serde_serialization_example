/*
    This file shows how to use the serde 'remote' trait
    in order to make an easy definition of a struct
    without the need for writing a full converter.

    Note: this remote won't work if the parent 
    type is used within a data structure such as
    a vector or map. In that case you need to use
    the newtype idiom, write a converter, and use
    the newtype as the data structure's generic type.
    See module 'vec2' for an example.

    Based on the documention: https://serde.rs/
*/

use serde::{Deserialize, Serialize};
use cgmath::{Vector4, Matrix4};

#[derive(Serialize, Deserialize)]
#[serde(remote = "Vector4<f32>")]
pub struct Vector4Remote { x: f32, y: f32, z: f32, w: f32, }

#[derive(Serialize, Deserialize)]
#[serde(remote = "Matrix4<f32>")]
pub struct Matrix4Remote { 
    #[serde(with = "self::Vector4Remote")]
    x: Vector4<f32>,

    #[serde(with = "self::Vector4Remote")] 
    y: Vector4<f32>, 

    #[serde(with = "self::Vector4Remote")]
    z: Vector4<f32>, 

    #[serde(with = "self::Vector4Remote")]
    w: Vector4<f32>, 
}