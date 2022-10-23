use specs::{Builder, World, WorldExt};

use crate::{
    component::{
        player_animation::PlayerAnimationComponent, player_physics::PlayerPhysicsComponent,
        player_weapon::PlayerWeaponComponent, position::PositionComponent, sprite::SpriteComponent,
        track_position::TrackPositionComponent,
    },
    sound::SoundId,
    sprite::{SpriteDescription, SpriteId},
    system::render::Layer,
    /* FIXME: do proper delta time implementation in physics systems */
    FRAME_RATE_GAME,
    /* FIXME: should not be used via constant (maybe have screen width / height as resource and a bounding box component?) */
    GAME_HEIGHT, GAME_WIDTH,
};

const VX_MAX: f32 = 0.000314 * GAME_WIDTH as f32 * (1000.0 / (FRAME_RATE_GAME as f32));
const VY_MAX: f32 = 0.000487 * GAME_HEIGHT as f32 * (1000.0 / (FRAME_RATE_GAME as f32));
const AX_MAX: f32 = 6.286_875e-5 * GAME_WIDTH as f32 * (1000.0 / (FRAME_RATE_GAME as f32));
const AY_MAX: f32 = 9.735e-5 * GAME_HEIGHT as f32 * (1000.0 / (FRAME_RATE_GAME as f32));

pub struct Player;

impl Player {
    #[allow(clippy::too_many_arguments)]
    pub fn create_player(
        world: &mut World,
        sprite_id: SpriteId,
        sprite_desc: &SpriteDescription,
        x: f32,
        y: f32,
        bullet_sprite_id: SpriteId,
        bullet_dimensions: (u32, u32),
        bullet_sound_id: SoundId,
        glow_sprite_id: SpriteId,
    ) {
        let player_entity = world
            .create_entity()
            .with(SpriteComponent::new(sprite_id, Layer::AirUnits))
            .with(PositionComponent::new(x, y))
            .with(PlayerPhysicsComponent {
                ax: 0.0,
                ay: 0.0,
                vx: 0.0,
                vy: 0.0,
                ax_max: AX_MAX,
                ay_max: AY_MAX,
                vx_max: VX_MAX,
                vy_max: VY_MAX,
                x_min: (25 + sprite_desc.frame_dimensions.0 / 2_usize) as f32,
                x_max: (GAME_WIDTH - 25 - sprite_desc.frame_dimensions.0 as u32 / 2) as f32,
                y_min: (25 + sprite_desc.frame_dimensions.1 / 2_usize) as f32,
                y_max: (GAME_HEIGHT - 25 - sprite_desc.frame_dimensions.1 as u32 / 2) as f32,
            })
            .with(PlayerAnimationComponent::default())
            .with(PlayerWeaponComponent::new(
                FRAME_RATE_GAME / 6,
                bullet_sprite_id,
                bullet_dimensions,
                bullet_sound_id,
            ))
            .build();

        world
            .create_entity()
            .with(SpriteComponent::new(glow_sprite_id, Layer::Effects).with_scale_factor(1.2))
            .with(PositionComponent::new(0.0, 0.0))
            .with(TrackPositionComponent {
                tracked_entity: player_entity,
                offset: (-0.0, -6.0),
            })
            .build();
    }
}
