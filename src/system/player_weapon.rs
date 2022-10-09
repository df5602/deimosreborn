use log::info;
use specs::{Builder, Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

use crate::{
    component::{
        bullet_physics::BulletPhysicsComponent, player_weapon::PlayerWeaponComponent,
        position::PositionComponent, sprite::SpriteComponent,
    },
    resource::player_input::PlayerInput,
    SCREEN_HEIGHT, SCREEN_WIDTH,
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
                    .with(BulletPhysicsComponent {
                        vx: 0.0,
                        vy: -10.0,
                        x_min: -32.0, // FIXME: need a better solution for bounding boxes, need to know sprite size here?
                        x_max: (SCREEN_WIDTH + 32) as f32,
                        y_min: -32.0,
                        y_max: (SCREEN_HEIGHT + 32) as f32,
                    })
                    .build();

                info!(target: "PlayerWeaponSystem", "Spawn bullet");
            }
        }
    }
}
