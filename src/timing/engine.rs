use std::time::{Instant, Duration};

const APPROACH_MS: u64 = 1000; // 1 second lead time

pub struct GameClock {
    start: Instant,
    paused_at: Option<Instant>,
    accumulated: Duration,
}

impl GameClock {
    pub fn new() -> Self {
        GameClock { start: Instant::now(), paused_at: None, accumulated: Duration::ZERO }
    }
    
    pub fn elapsed_ms(&self) -> u64 {
        let elapsed = if let Some(paused) = self.paused_at {
            paused.duration_since(self.start)
        } else {
            self.start.elapsed()
        };
        elapsed.as_millis() as u64
    }
    
    pub fn spawn_time_ms(&self, note_time_ms: u64) -> u64 {
        note_time_ms.saturating_sub(APPROACH_MS)
    }
    
    pub fn should_spawn(&self, spawn_time_ms: u64) -> bool {
        self.elapsed_ms() >= spawn_time_ms
    }
    
    pub fn should_miss(&self, note_time_ms: u64) -> bool {
        self.elapsed_ms() > note_time_ms + 150
    }
}
