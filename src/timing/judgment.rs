#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Judgment { Perfect, Great, Good, Miss }

pub struct HitWindow {
    pub perfect_ms: i64,
    pub great_ms: i64,
    pub good_ms: i64,
}

impl Default for HitWindow {
    fn default() -> Self {
        HitWindow { perfect_ms: 50, great_ms: 100, good_ms: 150 }
    }
}

pub fn judge(note_time_ms: u64, hit_time_ms: u64, window: &HitWindow) -> Judgment {
    let delta = (hit_time_ms as i64 - note_time_ms as i64).abs();
    if delta <= window.perfect_ms { Judgment::Perfect }
    else if delta <= window.great_ms { Judgment::Great }
    else if delta <= window.good_ms { Judgment::Good }
    else { Judgment::Miss }
}
