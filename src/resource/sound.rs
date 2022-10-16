use std::sync::{mpsc::Sender, Mutex};

use crate::sound::SoundId;

pub struct SoundSystem {
    // TODO: this is a workaround around `Chunks` not being Sync+Send, maybe there is a better way?
    pub sender: Mutex<Sender<SoundId>>,
}
