use crate::scoring::judgment::{Judgment, score_for_judgment};

pub struct GameStats {
    pub score: u64,
    pub combo: u32,
    pub max_combo: u32,
    pub perfects: u32,
    pub greats: u32,
    pub goods: u32,
    pub misses: u32,
    pub total_notes: u32,
}

impl GameStats {
    pub fn new() -> Self {
        GameStats {
            score: 0,
            combo: 0,
            max_combo: 0,
            perfects: 0,
            greats: 0,
            goods: 0,
            misses: 0,
            total_notes: 0,
        }
    }

    pub fn add_judgment(&mut self, j: &Judgment) {
        // Update combo and score accordingly
        match j {
            Judgment::Perfect | Judgment::Great | Judgment::Good => {
                self.combo += 1;
                if self.combo > self.max_combo {
                    self.max_combo = self.combo;
                }
            }
            Judgment::Miss => {
                self.combo = 0;
            }
        }

        // Update counts
        match j {
            Judgment::Perfect => self.perfects += 1,
            Judgment::Great => self.greats += 1,
            Judgment::Good => self.goods += 1,
            Judgment::Miss => self.misses += 1,
        }
        self.total_notes += 1;

        // Update score using current combo after applying increment/reset
        let add = score_for_judgment(j, self.combo);
        self.score += add;
        // total_notes updated above
    }

    pub fn accuracy(&self) -> f64 {
        if self.total_notes == 0 {
            return 0.0;
        }
        // Compute achieved points using the classic base scheme
        let achieved = (self.perfects as u64) * crate::scoring::judgment::PERFECT_BASE
            + (self.greats as u64) * crate::scoring::judgment::GREAT_BASE
            + (self.goods as u64) * crate::scoring::judgment::GOOD_BASE;
        (achieved as f64) / ((self.total_notes as f64) * 300.0) * 100.0
    }

    pub fn display(&self) -> String {
        format!("Score: {}  Combo: {}  MaxCombo: {}  Accuracy: {:.2}%", self.score, self.combo, self.max_combo, self.accuracy())
    }
}
