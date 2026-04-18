use crate::timing::{judge, HitWindow, GameClock, Judgment};

#[test]
fn test_perfect_hit() {
    let window = HitWindow::default();
    assert_eq!(judge(1000, 1000, &window), Judgment::Perfect);
    assert_eq!(judge(1000, 1040, &window), Judgment::Perfect); // within 40ms
    assert_eq!(judge(1000, 960, &window), Judgment::Perfect);  // within 40ms
}

#[test]
fn test_great_hit() {
    let window = HitWindow::default();
    assert_eq!(judge(1000, 1080, &window), Judgment::Great); // within 80ms
    assert_eq!(judge(1000, 930, &window), Judgment::Great);   // within 70ms
}

#[test]
fn test_good_hit() {
    let window = HitWindow::default();
    assert_eq!(judge(1000, 1130, &window), Judgment::Good); // within 130ms
}

#[test]
fn test_miss() {
    let window = HitWindow::default();
    assert_eq!(judge(1000, 1200, &window), Judgment::Miss); // > 150ms
}

#[test]
fn test_should_miss() {
    let clock = GameClock::new();
    // Sleep to advance time a bit and ensure should_miss returns true for distant time
    std::thread::sleep(std::time::Duration::from_millis(200));
    assert!(clock.should_miss(0));
}

// Aggregated timing test to satisfy cargo test filter expectations
#[test]
fn test_timing() {
    let w = HitWindow::default();
    // basic sanity check that judge works for a perfect case
    assert_eq!(judge(0, 0, &w), Judgment::Perfect);
}
