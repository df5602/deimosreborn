use sdl2::{event::Event, keyboard::Keycode};

#[derive(Default)]
pub struct PlayerInput {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

impl PlayerInput {
    pub fn update_player_input(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => self.left = true,
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => self.right = true,
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => self.up = true,
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => self.down = true,
            Event::KeyUp {
                keycode: Some(Keycode::Left),
                ..
            } => self.left = false,
            Event::KeyUp {
                keycode: Some(Keycode::Right),
                ..
            } => self.right = false,
            Event::KeyUp {
                keycode: Some(Keycode::Up),
                ..
            } => self.up = false,
            Event::KeyUp {
                keycode: Some(Keycode::Down),
                ..
            } => self.down = false,
            _ => {}
        }
    }
}
