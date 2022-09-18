use specs::{Component, VecStorage};

use crate::sprite::SpriteId;

pub struct SpriteComponent {
    pub sprite: SpriteId,
}

impl Component for SpriteComponent {
    type Storage = VecStorage<Self>;
}
