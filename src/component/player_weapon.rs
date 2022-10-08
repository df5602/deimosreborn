use specs::{Component, HashMapStorage};

use crate::sprite::SpriteId;

pub struct PlayerWeaponComponent {
    pub cooldown_reset: u32,
    pub cooldown: u32,
    pub bullet_sprite: SpriteId,
}

impl Component for PlayerWeaponComponent {
    type Storage = HashMapStorage<Self>;
}

impl PlayerWeaponComponent {
    pub fn new(cooldown: u32, bullet_sprite: SpriteId) -> Self {
        Self {
            cooldown_reset: cooldown,
            cooldown: 0,
            bullet_sprite,
        }
    }
}
