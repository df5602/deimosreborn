use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::component::{
    player_animation::{GlowAnimationState, PlayerAnimationComponent, PlayerAnimationState},
    player_physics::PlayerPhysicsComponent,
    sprite::SpriteComponent,
};

pub struct PlayerAnimationSystem;

impl<'sys> System<'sys> for PlayerAnimationSystem {
    type SystemData = (
        Entities<'sys>,
        WriteStorage<'sys, PlayerAnimationComponent>,
        ReadStorage<'sys, PlayerPhysicsComponent>,
        WriteStorage<'sys, SpriteComponent>,
    );

    fn run(&mut self, (entities, mut animation, physics, mut sprite): Self::SystemData) {
        for (e, animation, physics) in (&entities, &mut animation, &physics).join() {
            let player_sprite = sprite
                .get_mut(e)
                .expect("player entity should have a sprite");

            const ANIMATION_DURATION: usize = 1;

            animation.animation_state = match animation.animation_state {
                PlayerAnimationState::Neutral(n) if n > 0 => PlayerAnimationState::Neutral(n - 1),
                PlayerAnimationState::Left1(n) if n > 0 => PlayerAnimationState::Left1(n - 1),
                PlayerAnimationState::Left2(n) if n > 0 => PlayerAnimationState::Left2(n - 1),
                PlayerAnimationState::Left3(n) if n > 0 => PlayerAnimationState::Left3(n - 1),
                PlayerAnimationState::Right1(n) if n > 0 => PlayerAnimationState::Right1(n - 1),
                PlayerAnimationState::Right2(n) if n > 0 => PlayerAnimationState::Right2(n - 1),
                PlayerAnimationState::Right3(n) if n > 0 => PlayerAnimationState::Right3(n - 1),
                PlayerAnimationState::Neutral(n) => {
                    assert!(n == 0);
                    if physics.ax < 0.0 {
                        PlayerAnimationState::Left1(ANIMATION_DURATION)
                    } else if physics.ax > 0.0 {
                        PlayerAnimationState::Right1(ANIMATION_DURATION)
                    } else {
                        PlayerAnimationState::Neutral(0)
                    }
                }
                PlayerAnimationState::Left1(n) => {
                    assert!(n == 0);
                    if physics.ax < 0.0 {
                        PlayerAnimationState::Left2(ANIMATION_DURATION)
                    } else if physics.ax > 0.0 || physics.vx == 0.0 {
                        PlayerAnimationState::Neutral(ANIMATION_DURATION)
                    } else {
                        PlayerAnimationState::Left1(0)
                    }
                }
                PlayerAnimationState::Left2(n) => {
                    assert!(n == 0);
                    if physics.ax < 0.0 {
                        PlayerAnimationState::Left3(ANIMATION_DURATION)
                    } else if physics.ax > 0.0 || physics.vx == 0.0 {
                        PlayerAnimationState::Left1(ANIMATION_DURATION)
                    } else {
                        PlayerAnimationState::Left2(0)
                    }
                }
                PlayerAnimationState::Left3(n) => {
                    assert!(n == 0);
                    if physics.ax < 0.0 {
                        PlayerAnimationState::Left3(0)
                    } else if physics.ax > 0.0 || physics.vx == 0.0 {
                        PlayerAnimationState::Left2(ANIMATION_DURATION)
                    } else {
                        PlayerAnimationState::Left3(0)
                    }
                }
                PlayerAnimationState::Right1(n) => {
                    assert!(n == 0);
                    if physics.ax < 0.0 {
                        PlayerAnimationState::Neutral(ANIMATION_DURATION)
                    } else if physics.ax > 0.0 {
                        PlayerAnimationState::Right2(ANIMATION_DURATION)
                    } else if physics.vx == 0.0 {
                        PlayerAnimationState::Neutral(ANIMATION_DURATION)
                    } else {
                        PlayerAnimationState::Right1(0)
                    }
                }
                PlayerAnimationState::Right2(n) => {
                    assert!(n == 0);
                    if physics.ax < 0.0 {
                        PlayerAnimationState::Right1(ANIMATION_DURATION)
                    } else if physics.ax > 0.0 {
                        PlayerAnimationState::Right3(ANIMATION_DURATION)
                    } else if physics.vx == 0.0 {
                        PlayerAnimationState::Right1(ANIMATION_DURATION)
                    } else {
                        PlayerAnimationState::Right2(0)
                    }
                }
                PlayerAnimationState::Right3(n) => {
                    assert!(n == 0);
                    if physics.ax < 0.0 {
                        PlayerAnimationState::Right2(ANIMATION_DURATION)
                    } else if physics.ax > 0.0 {
                        PlayerAnimationState::Right3(0)
                    } else if physics.vx == 0.0 {
                        PlayerAnimationState::Right2(ANIMATION_DURATION)
                    } else {
                        PlayerAnimationState::Right3(0)
                    }
                }
            };

            player_sprite.current_frame_idx = match animation.animation_state {
                PlayerAnimationState::Neutral(_) => 0,
                PlayerAnimationState::Left1(_) => 1,
                PlayerAnimationState::Left2(_) => 2,
                PlayerAnimationState::Left3(_) => 3,
                PlayerAnimationState::Right1(_) => 4,
                PlayerAnimationState::Right2(_) => 5,
                PlayerAnimationState::Right3(_) => 6,
            };

            if let Some(glow_entity) = animation.weapon_glow_entity {
                let glow_sprite = sprite
                    .get_mut(glow_entity)
                    .expect("glow entity should be alive");

                const GLOW_ANIMATION_DURATION: usize = 2;
                animation.glow_animation_state = match animation.glow_animation_state {
                    GlowAnimationState::Off => GlowAnimationState::Off,
                    GlowAnimationState::Fire => GlowAnimationState::Fire1(GLOW_ANIMATION_DURATION),
                    GlowAnimationState::Fire1(n) if n > 0 => GlowAnimationState::Fire1(n - 1),
                    GlowAnimationState::Fire2(n) if n > 0 => GlowAnimationState::Fire2(n - 1),
                    GlowAnimationState::Fire3(n) if n > 0 => GlowAnimationState::Fire3(n - 1),
                    GlowAnimationState::Cooldown1(n) if n > 0 => {
                        GlowAnimationState::Cooldown1(n - 1)
                    }
                    GlowAnimationState::Cooldown2(n) if n > 0 => {
                        GlowAnimationState::Cooldown2(n - 1)
                    }
                    GlowAnimationState::Cooldown3(n) if n > 0 => {
                        GlowAnimationState::Cooldown3(n - 1)
                    }
                    GlowAnimationState::Fire1(n) => {
                        assert!(n == 0);
                        GlowAnimationState::Fire2(GLOW_ANIMATION_DURATION)
                    }
                    GlowAnimationState::Fire2(n) => {
                        assert!(n == 0);
                        GlowAnimationState::Fire3(GLOW_ANIMATION_DURATION)
                    }
                    GlowAnimationState::Fire3(n) => {
                        assert!(n == 0);
                        GlowAnimationState::Cooldown1(GLOW_ANIMATION_DURATION)
                    }
                    GlowAnimationState::Cooldown1(n) => {
                        assert!(n == 0);
                        GlowAnimationState::Cooldown2(GLOW_ANIMATION_DURATION)
                    }
                    GlowAnimationState::Cooldown2(n) => {
                        assert!(n == 0);
                        GlowAnimationState::Cooldown3(GLOW_ANIMATION_DURATION)
                    }
                    GlowAnimationState::Cooldown3(n) => {
                        assert!(n == 0);
                        GlowAnimationState::Off
                    }
                };

                glow_sprite.scale_factor = match animation.glow_animation_state {
                    GlowAnimationState::Off => 0.5,
                    GlowAnimationState::Fire => 1.0, /* should be unreachable */
                    GlowAnimationState::Fire1(_) => 1.0,
                    GlowAnimationState::Fire2(_) => 1.1,
                    GlowAnimationState::Fire3(_) => 1.2,
                    GlowAnimationState::Cooldown1(_) => 1.1,
                    GlowAnimationState::Cooldown2(_) => 1.0,
                    GlowAnimationState::Cooldown3(_) => 0.9,
                };
            }
        }
    }
}
