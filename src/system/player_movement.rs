use specs::{Join, Read, System, WriteStorage};

use crate::{
    component::{player_physics::PlayerPhysicsComponent, position::PositionComponent},
    resource::player_input::PlayerInput,
};

pub struct PlayerMovementSystem;

impl<'sys> System<'sys> for PlayerMovementSystem {
    type SystemData = (
        Read<'sys, PlayerInput>,
        WriteStorage<'sys, PlayerPhysicsComponent>,
        WriteStorage<'sys, PositionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_input, mut physics, mut position) = data;

        for (physics, position) in (&mut physics, &mut position).join() {
            // Acceleration:
            if player_input.left {
                physics.ax = -physics.ax_max;
            } else if player_input.right {
                physics.ax = physics.ax_max;
            } else if physics.vx < 0.0 {
                physics.ax = f32::min(physics.ax_max, -physics.vx);
            } else if physics.vx > 0.0 {
                physics.ax = f32::max(-physics.ax_max, -physics.vx);
            } else {
                physics.ax = 0.0;
            }

            if player_input.up {
                physics.ay = -physics.ay_max;
            } else if player_input.down {
                physics.ay = physics.ay_max;
            } else if physics.vy < 0.0 {
                physics.ay = f32::min(physics.ay_max, -physics.vy);
            } else if physics.vy > 0.0 {
                physics.ay = f32::max(-physics.ay_max, -physics.vy);
            } else {
                physics.ay = 0.0;
            }

            // Velocity:
            physics.vx += physics.ax;
            if physics.vx > physics.vx_max {
                physics.vx = physics.vx_max;
            } else if physics.vx < -physics.vx_max {
                physics.vx = -physics.vx_max;
            }

            physics.vy += physics.ay;
            if physics.vy > physics.vy_max {
                physics.vy = physics.vy_max;
            } else if physics.vy < -physics.vy_max {
                physics.vy = -physics.vy_max;
            }

            // Position:
            position.x += physics.vx;
            if position.x < physics.x_min {
                position.x = physics.x_min;
            } else if position.x > physics.x_max {
                position.x = physics.x_max;
            }

            position.y += physics.vy;
            if position.y < physics.y_min {
                position.y = physics.y_min;
            } else if position.y > physics.y_max {
                position.y = physics.y_max;
            }
        }
    }
}
