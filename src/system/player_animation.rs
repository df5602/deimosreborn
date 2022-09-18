use specs::{Join, ReadStorage, System, WriteStorage};

use crate::component::{
    player_animation::{PlayerAnimationComponent, PlayerAnimationState},
    player_physics::PlayerPhysicsComponent,
};

pub struct PlayerAnimationSystem;

impl<'sys> System<'sys> for PlayerAnimationSystem {
    type SystemData = (
        WriteStorage<'sys, PlayerAnimationComponent>,
        ReadStorage<'sys, PlayerPhysicsComponent>,
    );

    fn run(&mut self, (mut animation, physics): Self::SystemData) {
        for (animation, physics) in (&mut animation, &physics).join() {
            const ANIMATION_DURATION: usize = 2;

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

            animation.sprite_frame_idx = match animation.animation_state {
                PlayerAnimationState::Neutral(_) => 0,
                PlayerAnimationState::Left1(_) => 1,
                PlayerAnimationState::Left2(_) => 2,
                PlayerAnimationState::Left3(_) => 3,
                PlayerAnimationState::Right1(_) => 4,
                PlayerAnimationState::Right2(_) => 5,
                PlayerAnimationState::Right3(_) => 6,
            };
        }
    }
}
