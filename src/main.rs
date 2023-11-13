use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use cgmath::{Vector2, Vector3, Matrix4, SquareMatrix};

mod remote;

#[derive(Serialize, Deserialize, Debug)]
struct Square {
    width: f32,
    height: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    #[serde(skip)]
    x: i32,

    #[serde(skip)]
    y: i32,

    my_square: Square,
    my_vec: Vec<Vector2<f32>>,
    my_map: HashMap<u32, Vector2<f32>>,
    v3: Vector3<f32>,

    // Note this attribute isn't needed, but shows
    // how to use serde's 'remote with'
    #[serde(with = "remote::Matrix4Remote")]
    m4: Matrix4<f32>,
}

fn main() {
    let point = Point { 
        x: 10, 
        y: 12,
        my_square: Square { 
            width: 33.45, 
            height: 12.35 
        },
        my_vec: vec![Vector2::new(10.5, 22.3), Vector2::new(100.5, 220.3)],
        my_map: HashMap::from([(1, Vector2::new(1000.5, 202.3))]),
        v3: Vector3::new(10.4, 5.6, 33.2),
        m4: Matrix4::identity(),
    };

    // Json
    {
        //let serialized = serde_json::to_string(&point).unwrap();
        let serialized = serde_json::ser::to_string_pretty(&point).unwrap();
        println!("JSON Serialized: {}", serialized);
        _ = std::fs::write(".\\data\\data.json", &serialized);

        let deserialize: Point = serde_json::from_str(&serialized).unwrap();
        println!("JSON Deserialized: {:?}", deserialize);
        println!("\n");
    }

    // Ron
    {
        //let serialized = ron::to_string(&point).unwrap();
        let serialized = ron::ser::to_string_pretty(&point, ron::ser::PrettyConfig::default()).unwrap();
        println!("RON Serialize: {}", serialized);
        _ = std::fs::write(".\\data\\data.ron", &serialized);

        let deserialize: Point = ron::from_str(&serialized).unwrap();
        println!("RON Deserialized: {:?}", deserialize);
        println!("\n");
    }
}
