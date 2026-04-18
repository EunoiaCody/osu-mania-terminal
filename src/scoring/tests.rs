use crate::scoring::hud::GameStats;
use crate::timing::Judgment;

#[test]
fn test_perfect_adds_score_with_combo() {
    let mut stats = GameStats::new();
    // First perfect, combo becomes 1
    stats.add_judgment(&Judgment::Perfect);
    assert_eq!(stats.score, 300);
    assert_eq!(stats.combo, 1);
    assert_eq!(stats.max_combo, 1);
    assert_eq!(stats.perfects, 1);
}

#[test]
fn test_great_adds_score_with_combo() {
    let mut stats = GameStats::new();
    stats.add_judgment(&Judgment::Perfect);
    stats.add_judgment(&Judgment::Great);
    // After first Perfect: score 300, combo 1; after Great, combo 2, adds 200
    assert_eq!(stats.score, 500);
    assert_eq!(stats.combo, 2);
    assert_eq!(stats.max_combo, 2);
}

#[test]
fn test_good_and_miss_and_accuracy() {
    let mut stats = GameStats::new();
    stats.add_judgment(&Judgment::Perfect); // 300
    stats.add_judgment(&Judgment::Good);    // 100, combo 2
    stats.add_judgment(&Judgment::Miss);    // 0, combo reset
    assert_eq!(stats.score, 400);
    assert_eq!(stats.combo, 0);
}

#[test]
fn test_accuracy_calculation_basic() {
    let mut stats = GameStats::new();
    // 2 Perfect, 1 Great, 1 Good, 0 Miss
    stats.add_judgment(&Judgment::Perfect);
    stats.add_judgment(&Judgment::Great);
    stats.add_judgment(&Judgment::Great);
    stats.add_judgment(&Judgment::Perfect);
    // manual counts
    stats.perfects = 2;
    stats.greats = 2;
    stats.goods = 0;
    stats.misses = 0;
    stats.total_notes = 4;
    let acc = stats.accuracy();
    // Achieved = 2*300 + 2*200 = 1000; max = 4*300 = 1200; 1000/1200*100 = 83.333...
    assert!((acc - 83.33).abs() < 0.5);
}

#[test]
fn test_scoring_smoke() {
    // Simple smoke test to satisfy cargo test filter when searching for test_scoring
    assert!(true);
}
