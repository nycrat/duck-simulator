#[derive(Debug)]
pub struct Duck {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation_radians: f32,
    pub score: u32,
    pub name: Option<String>,
    pub variety: Option<String>,
    pub color: Option<String>,
}

impl Duck {
    pub fn new() -> Self {
        Duck {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rotation_radians: 0.0,
            score: 0,
            name: None,
            variety: None,
            color: None,
        }
    }
}
