use specs::{Component, VecStorage};

use crate::sprite::SpriteId;

pub struct SpriteComponent {
    pub sprite: SpriteId,
    pub current_frame_idx: usize,
}

impl SpriteComponent {
    pub fn new(sprite_id: SpriteId) -> Self {
        Self {
            sprite: sprite_id,
            current_frame_idx: 0,
        }
    }
}

impl Component for SpriteComponent {
    type Storage = VecStorage<Self>;
}
