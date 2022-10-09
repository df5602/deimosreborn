use specs::{Component, VecStorage};

#[derive(Copy, Clone)]
pub struct PositionComponent {
    x_n: f32,
    y_n: f32,
    x_p: f32,
    y_p: f32,
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

    pub fn update_x(&mut self, x: f32) {
        self.x_p = self.x_n;
        self.x_n = x;
    }

    #[allow(dead_code)]
    pub fn reset_x(&mut self, x: f32) {
        self.x_p = x;
        self.x_n = x;
    }

    pub fn update_y(&mut self, y: f32) {
        self.y_p = self.y_n;
        self.y_n = y;
    }

    #[allow(dead_code)]
    pub fn reset_y(&mut self, y: f32) {
        self.y_p = y;
        self.y_n = y;
    }

    pub fn x(&self) -> f32 {
        self.x_n
    }

    pub fn y(&self) -> f32 {
        self.y_n
    }

    pub fn previous_x(&self) -> f32 {
        self.x_p
    }

    pub fn previous_y(&self) -> f32 {
        self.y_p
    }
}
