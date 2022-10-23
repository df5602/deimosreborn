use specs::{Component, VecStorage};

use crate::{sprite::SpriteId, system::render::Layer};

pub struct SpriteComponent {
    pub sprite: SpriteId,
    pub layer: Layer,
    pub current_frame_idx: usize,
    pub scale_factor: f32,
}

impl SpriteComponent {
    pub fn new(sprite_id: SpriteId, layer: Layer) -> Self {
        Self {
            sprite: sprite_id,
            layer,
            current_frame_idx: 0,
            scale_factor: 1.0,
        }
    }

    pub fn with_scale_factor(mut self, scale_factor: f32) -> Self {
        self.scale_factor = scale_factor;
        self
    }
}

impl Component for SpriteComponent {
    type Storage = VecStorage<Self>;
}
