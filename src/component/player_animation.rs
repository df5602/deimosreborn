use specs::{Component, Entity, HashMapStorage};

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

#[derive(Debug, Copy, Clone)]
pub enum GlowAnimationState {
    Off,
    Fire,
    Fire1(usize),
    Fire2(usize),
    Fire3(usize),
    Cooldown1(usize),
    Cooldown2(usize),
    Cooldown3(usize),
}

pub struct PlayerAnimationComponent {
    pub animation_state: PlayerAnimationState,
    pub weapon_glow_entity: Option<Entity>,
    pub glow_animation_state: GlowAnimationState,
}

impl Default for PlayerAnimationComponent {
    fn default() -> Self {
        Self {
            animation_state: PlayerAnimationState::Neutral(0),
            weapon_glow_entity: None,
            glow_animation_state: GlowAnimationState::Off,
        }
    }
}

impl Component for PlayerAnimationComponent {
    type Storage = HashMapStorage<Self>;
}
