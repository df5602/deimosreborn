use log::{error, info};
use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::component::{bullet_physics::BulletPhysicsComponent, position::PositionComponent};

pub struct BulletPhysicsSystem;

impl<'sys> System<'sys> for BulletPhysicsSystem {
    type SystemData = (
        Entities<'sys>,
        ReadStorage<'sys, BulletPhysicsComponent>,
        WriteStorage<'sys, PositionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, bullet_physics, mut position) = data;

        for (e, bullet_physics, position) in (&entities, &bullet_physics, &mut position).join() {
            let x = position.x() + bullet_physics.vx;
            let y = position.y() + bullet_physics.vy;

            if x < bullet_physics.x_min
                || x > bullet_physics.x_max
                || y < bullet_physics.y_min
                || y > bullet_physics.y_max
            {
                let err = entities.delete(e);
                if let Err(e) = err {
                    error!(target: "BulletPhysicsSystem", "{}", e);
                } else {
                    info!(target: "BulletPhysicsSystem", "Bullet left active area, delete it");
                }
            }

            position.update_x(x);
            position.update_y(y);
        }
    }
}
