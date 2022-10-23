use specs::{Component, Entity, HashMapStorage};

pub struct TrackPositionComponent {
    pub tracked_entity: Entity,
    pub offset: (f32, f32),
}

impl Component for TrackPositionComponent {
    type Storage = HashMapStorage<Self>;
}
