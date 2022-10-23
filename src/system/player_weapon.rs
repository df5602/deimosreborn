use log::info;
use specs::{
    Builder, Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage,
};

use crate::{
    component::{
        bullet_physics::BulletPhysicsComponent, player_weapon::PlayerWeaponComponent,
        position::PositionComponent, sprite::SpriteComponent,
    },
    resource::{player_input::PlayerInput, sound::AudioInterface},
    system::render::Layer,
    FRAME_RATE_GAME, GAME_HEIGHT, GAME_WIDTH,
};

const VY: f32 = -0.000625 * GAME_HEIGHT as f32 * (1000.0 / (FRAME_RATE_GAME as f32));
const POS_OFFSET: f32 = 0.007292 * GAME_WIDTH as f32;

pub struct PlayerWeaponSystem;

impl PlayerWeaponSystem {
    fn spawn_bullet<B>(builder: B, position: PositionComponent, weapon: &PlayerWeaponComponent)
    where
        B: Builder,
    {
        builder
            .with(SpriteComponent::new(weapon.bullet_sprite, Layer::Effects))
            .with(position)
            .with(BulletPhysicsComponent {
                vx: 0.0,
                vy: VY,
                x_min: -(weapon.bullet_dimensions.0 as f32 / 2.0),
                x_max: GAME_WIDTH as f32 + weapon.bullet_dimensions.0 as f32 / 2.0,
                y_min: -(weapon.bullet_dimensions.1 as f32 / 2.0),
                y_max: GAME_HEIGHT as f32 + weapon.bullet_dimensions.1 as f32 / 2.0,
            })
            .build();
    }
}

impl<'sys> System<'sys> for PlayerWeaponSystem {
    type SystemData = (
        Read<'sys, PlayerInput>,
        ReadExpect<'sys, AudioInterface>,
        Entities<'sys>,
        Read<'sys, LazyUpdate>,
        WriteStorage<'sys, PlayerWeaponComponent>,
        ReadStorage<'sys, PositionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_input, audio, entities, lazy_update, mut weapon, position) = data;

        // TODO: glow

        for (weapon, position) in (&mut weapon, &position).join() {
            if weapon.cooldown > 0 {
                weapon.cooldown -= 1;
            } else if player_input.shoot_air {
                weapon.cooldown += weapon.cooldown_reset;

                let mut position_bullet_left = *position;
                let left_x = position_bullet_left.x() - POS_OFFSET;
                position_bullet_left.reset_x(left_x);

                let mut position_bullet_right = *position;
                let right_x = position_bullet_right.x() + POS_OFFSET;
                position_bullet_right.reset_x(right_x);

                PlayerWeaponSystem::spawn_bullet(
                    lazy_update.create_entity(&entities),
                    position_bullet_left,
                    weapon,
                );

                PlayerWeaponSystem::spawn_bullet(
                    lazy_update.create_entity(&entities),
                    position_bullet_right,
                    weapon,
                );

                audio.play_sound(weapon.bullet_sound);

                info!(target: "PlayerWeaponSystem", "Spawn bullets");
            }
        }
    }
}
