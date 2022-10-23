use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::component::{position::PositionComponent, track_position::TrackPositionComponent};

pub struct PositionTrackSystem;

impl<'sys> System<'sys> for PositionTrackSystem {
    type SystemData = (
        Entities<'sys>,
        ReadStorage<'sys, TrackPositionComponent>,
        WriteStorage<'sys, PositionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, track, mut position) = data;

        for (e, track) in (&entities, &track).join() {
            let tracked_position = *position
                .get(track.tracked_entity)
                .expect("tracked entity should be alive");

            let this_position = position.get_mut(e).expect("current entity should be alive");

            // "Double update", so that previous and current position are correct
            // TODO: PositionComponent could maybe provide a nicer API for this
            this_position.update_x(tracked_position.previous_x() + track.offset.0);
            this_position.update_x(tracked_position.x() + track.offset.0);
            this_position.update_y(tracked_position.previous_y() + track.offset.1);
            this_position.update_y(tracked_position.y() + track.offset.1);
        }
    }
}
