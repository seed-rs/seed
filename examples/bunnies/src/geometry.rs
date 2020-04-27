use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        let x: f64 = rng.gen(); // random number in range [0, 1)
        let y: f64 = rng.gen(); // random number in range [0, 1)

        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Area {
    pub width: u32,
    pub height: u32,
}

pub const QUAD_GEOM_UNIT: [f32; 8] = [
    0.0, 1.0, // top-left
    0.0, 0.0, //bottom-left
    1.0, 1.0, // top-right
    1.0, 0.0, // bottom-right
];
