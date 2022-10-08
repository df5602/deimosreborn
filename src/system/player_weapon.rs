use specs::{Builder, Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

use crate::{
    component::{
        player_weapon::PlayerWeaponComponent, position::PositionComponent, sprite::SpriteComponent,
    },
    resource::player_input::PlayerInput,
};

pub struct PlayerWeaponSystem;

impl<'sys> System<'sys> for PlayerWeaponSystem {
    type SystemData = (
        Read<'sys, PlayerInput>,
        Entities<'sys>,
        Read<'sys, LazyUpdate>,
        WriteStorage<'sys, PlayerWeaponComponent>,
        ReadStorage<'sys, PositionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_input, entities, lazy_update, mut weapon, position) = data;

        for (weapon, position) in (&mut weapon, &position).join() {
            if weapon.cooldown > 0 {
                weapon.cooldown -= 1;
            } else if player_input.shoot_air {
                weapon.cooldown += weapon.cooldown_reset;

                lazy_update
                    .create_entity(&entities)
                    .with(SpriteComponent::new(weapon.bullet_sprite))
                    .with(*position)
                    .build();
            }
        }
    }
}
