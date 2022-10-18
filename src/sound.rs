use sdl2::mixer::Chunk;

#[derive(Debug, Copy, Clone)]
pub struct SoundId(usize);

pub struct SoundLibrary {
    sounds: Vec<Chunk>,
}

impl SoundLibrary {
    pub fn new() -> Self {
        Self { sounds: Vec::new() }
    }

    pub fn insert(&mut self, sound: Chunk) -> SoundId {
        self.sounds.push(sound);
        SoundId(self.sounds.len() - 1)
    }

    pub fn get(&self, id: SoundId) -> &Chunk {
        &self.sounds[id.0]
    }
}
