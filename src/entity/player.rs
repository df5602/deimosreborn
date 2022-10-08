use specs::{Builder, World, WorldExt};

use crate::{
    component::{
        player_animation::PlayerAnimationComponent, player_physics::PlayerPhysicsComponent,
        position::PositionComponent, sprite::SpriteComponent,
    },
    sprite::{SpriteDescription, SpriteId},
    /* FIXME: do proper delta time implementation in physics systems */
    FRAME_RATE_GAME,
    /* FIXME: should not be used via constant (maybe have screen width / height as resource and a bounding box component?) */
    SCREEN_HEIGHT, SCREEN_WIDTH,
};

const VX_MAX: f32 = 0.000314 * SCREEN_WIDTH as f32 * (1000.0 / (FRAME_RATE_GAME as f32));
const VY_MAX: f32 = 0.000487 * SCREEN_HEIGHT as f32 * (1000.0 / (FRAME_RATE_GAME as f32));
const AX_MAX: f32 = 6.286_875e-5 * SCREEN_WIDTH as f32 * (1000.0 / (FRAME_RATE_GAME as f32));
const AY_MAX: f32 = 9.735e-5 * SCREEN_HEIGHT as f32 * (1000.0 / (FRAME_RATE_GAME as f32));

pub struct Player;

impl Player {
    pub fn create_player(
        world: &mut World,
        sprite_id: SpriteId,
        sprite_desc: &SpriteDescription,
        x: f32,
        y: f32,
    ) {
        world
            .create_entity()
            .with(SpriteComponent::new(sprite_id))
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
                x_min: (50 + sprite_desc.frame_dimensions.0) as f32,
                x_max: (SCREEN_WIDTH - 50 - sprite_desc.frame_dimensions.0 as u32) as f32,
                y_min: (50 + sprite_desc.frame_dimensions.1) as f32,
                y_max: (SCREEN_HEIGHT - 50 - sprite_desc.frame_dimensions.1 as u32) as f32,
            })
            .with(PlayerAnimationComponent::default())
            .build();
    }
}
