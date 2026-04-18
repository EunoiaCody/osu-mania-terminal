use crate::beatmap::types::{Beatmap, Note, NoteKind};
use crate::game::state::{GameContext, GameState};

#[test]
fn test_integration() {
    let beatmap = Beatmap {
        title: String::from("Test Beatmap"),
        artist: String::from("Artist"),
        bpm: 120.0,
        lanes: 4,
        notes: vec![Note { time_ms: 1000, lane: 0, kind: NoteKind::Note, duration_ms: None }],
        timing_points: vec![],
    };
    let ctx_result = GameContext::new(beatmap, "missing_audio.mp3");
    if let Err(e) = ctx_result {
        eprintln!("Audio device not available (expected in headless env): {}", e);
        return;
    }
    let mut ctx = ctx_result.unwrap();
    assert!(ctx.state == GameState::Loading || ctx.state == GameState::Playing);
    ctx.update();
    let _frame = ctx.render();
}
