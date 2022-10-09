use specs::{Component, VecStorage};

pub struct BulletPhysicsComponent {
    pub vx: f32,
    pub vy: f32,

    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

impl Component for BulletPhysicsComponent {
    type Storage = VecStorage<Self>;
}
