use specs::{Component, HashMapStorage};

#[derive(Debug, Copy, Clone)]
pub enum PlayerAnimationState {
    Neutral(usize),
    Left1(usize),
    Left2(usize),
    Left3(usize),
    Right1(usize),
    Right2(usize),
    Right3(usize),
}

pub struct PlayerAnimationComponent {
    pub animation_state: PlayerAnimationState,
}

impl Default for PlayerAnimationComponent {
    fn default() -> Self {
        Self {
            animation_state: PlayerAnimationState::Neutral(0),
        }
    }
}

impl Component for PlayerAnimationComponent {
    type Storage = HashMapStorage<Self>;
}
