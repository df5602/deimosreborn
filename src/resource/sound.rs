use std::sync::{mpsc::Sender, Mutex};

use crate::sound::SoundId;

pub struct AudioInterface {
    // TODO: this is a workaround around `Chunks` not being Sync+Send, maybe there is a better way?
    sender: Mutex<Sender<SoundId>>,
}

impl AudioInterface {
    pub fn new(sender: Sender<SoundId>) -> Self {
        Self {
            sender: Mutex::new(sender),
        }
    }

    pub fn play_sound(&self, sound: SoundId) {
        self.sender
            .lock()
            .expect("mutex should be valid")
            .send(sound)
            .expect("channel should be valid");
    }
}
