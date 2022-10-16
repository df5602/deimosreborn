use log::info;
use specs::{
    Builder, Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage,
};

use crate::{
    component::{
        bullet_physics::BulletPhysicsComponent, player_weapon::PlayerWeaponComponent,
        position::PositionComponent, sprite::SpriteComponent,
    },
    resource::{player_input::PlayerInput, sound::SoundSystem},
    system::render::Layer,
    FRAME_RATE_GAME, SCREEN_HEIGHT, SCREEN_WIDTH,
};

const VY: f32 = -0.000625 * SCREEN_HEIGHT as f32 * (1000.0 / (FRAME_RATE_GAME as f32));
const POS_OFFSET: f32 = 0.007292 * SCREEN_WIDTH as f32;

pub struct PlayerWeaponSystem;

impl<'sys> System<'sys> for PlayerWeaponSystem {
    type SystemData = (
        Read<'sys, PlayerInput>,
        ReadExpect<'sys, SoundSystem>,
        Entities<'sys>,
        Read<'sys, LazyUpdate>,
        WriteStorage<'sys, PlayerWeaponComponent>,
        ReadStorage<'sys, PositionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_input, sound, entities, lazy_update, mut weapon, position) = data;

        // TODO: glow
        // TODO: SFX
        // FIXME: this probably needs to be organised better

        for (weapon, position) in (&mut weapon, &position).join() {
            if weapon.cooldown > 0 {
                weapon.cooldown -= 1;
            } else if player_input.shoot_air {
                weapon.cooldown += weapon.cooldown_reset;

                let mut bullet_left = *position;
                let left_x = bullet_left.x() - POS_OFFSET;
                bullet_left.reset_x(left_x);

                let mut bullet_right = *position;
                let right_x = bullet_right.x() + POS_OFFSET;
                bullet_right.reset_x(right_x);

                lazy_update
                    .create_entity(&entities)
                    .with(SpriteComponent::new(weapon.bullet_sprite, Layer::Effects))
                    .with(bullet_left)
                    .with(BulletPhysicsComponent {
                        vx: 0.0,
                        vy: VY,
                        x_min: -32.0, // FIXME: need a better solution for bounding boxes, need to know sprite size here?
                        x_max: (SCREEN_WIDTH + 32) as f32,
                        y_min: -32.0,
                        y_max: (SCREEN_HEIGHT + 32) as f32,
                    })
                    .build();

                lazy_update
                    .create_entity(&entities)
                    .with(SpriteComponent::new(weapon.bullet_sprite, Layer::Effects))
                    .with(bullet_right)
                    .with(BulletPhysicsComponent {
                        vx: 0.0,
                        vy: VY,
                        x_min: -32.0, // FIXME: need a better solution for bounding boxes, need to know sprite size here?
                        x_max: (SCREEN_WIDTH + 32) as f32,
                        y_min: -32.0,
                        y_max: (SCREEN_HEIGHT + 32) as f32,
                    })
                    .build();

                sound
                    .sender
                    .lock()
                    .expect("mutex is valid")
                    .send(weapon.bullet_sound)
                    .expect("channel is valid");

                info!(target: "PlayerWeaponSystem", "Spawn bullets");
            }
        }
    }
}
