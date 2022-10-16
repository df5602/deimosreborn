use sdl2::mixer::Chunk;

#[derive(Debug, Copy, Clone)]
pub struct SoundId(usize);

pub struct SoundManager {
    sounds: Vec<Chunk>,
}

// TODO: maybe "library" is a better name? (same for sprites)
impl SoundManager {
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
