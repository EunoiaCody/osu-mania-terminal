// Re-export the timing Judgment type for external use
pub use crate::timing::Judgment;

// Base scores
pub const PERFECT_BASE: u64 = 300;
pub const GREAT_BASE: u64 = 200;
pub const GOOD_BASE: u64 = 100;
pub const MISS_BASE: u64 = 0;

#[derive(Debug, Clone, PartialEq, Eq)]
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

pub fn score_for_judgment(j: &Judgment, combo: u32) -> u64 {
    let mult = 1u64 + (combo as u64 / 100);
    match j {
        Judgment::Perfect => PERFECT_BASE * mult,
        Judgment::Great => GREAT_BASE * mult,
        Judgment::Good => GOOD_BASE * mult,
        Judgment::Miss => MISS_BASE,
    }
}
