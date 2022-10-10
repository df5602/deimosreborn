use specs::{Component, VecStorage};

use crate::{sprite::SpriteId, system::render::Layer};

pub struct SpriteComponent {
    pub sprite: SpriteId,
    pub layer: Layer,
    pub current_frame_idx: usize,
}

impl SpriteComponent {
    pub fn new(sprite_id: SpriteId, layer: Layer) -> Self {
        Self {
            sprite: sprite_id,
            layer,
            current_frame_idx: 0,
        }
    }
}

impl Component for SpriteComponent {
    type Storage = VecStorage<Self>;
}
