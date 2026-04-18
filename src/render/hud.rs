use crate::scoring::GameStats;

pub struct HUD {
    pub bpm: f64,
}

impl HUD {
    pub fn new() -> Self { HUD { bpm: 120.0 } }

    pub fn render(&self, _stats: &GameStats, bpm: f64) -> String {
        // Simple HUD string
        format!("Score: {}  Combo: {}  BPM: {:.1}", 0, 0, bpm)
    }
}
