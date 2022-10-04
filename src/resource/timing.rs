use std::time::{Duration, Instant};

pub struct Timing {
    pub physics_tick: Instant,
    pub next_vsync: Option<Instant>,
    pub delta_time: Duration,
}

impl Default for Timing {
    fn default() -> Self {
        Self {
            physics_tick: Instant::now(),
            next_vsync: None,
            delta_time: Duration::from_secs(0),
        }
    }
}
