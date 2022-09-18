use specs::{Component, HashMapStorage};

pub struct PlayerPhysicsComponent {
    // Acceleration / velocity
    pub ax: f32,
    pub ay: f32,
    pub vx: f32,
    pub vy: f32,

    // Max. acceleration / velocity
    pub ax_max: f32,
    pub ay_max: f32,
    pub vx_max: f32,
    pub vy_max: f32,

    // Bounding box (should probably be separate component)
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

impl Component for PlayerPhysicsComponent {
    type Storage = HashMapStorage<Self>;
}
