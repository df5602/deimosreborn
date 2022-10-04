use specs::{Component, VecStorage};

pub struct PositionComponent {
    pub x_n: f32,
    pub y_n: f32,
    pub x_p: f32,
    pub y_p: f32,
}

impl Component for PositionComponent {
    type Storage = VecStorage<Self>;
}

impl PositionComponent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x_n: x,
            y_n: y,
            x_p: x,
            y_p: y,
        }
    }
}
