use std::path::Path;

use crate::beatmap::types::{Beatmap, Note, NoteKind};
use crate::timing::{GameClock, Judgment};
use crate::input::InputHandler;
use crate::audio::AudioPlayer;
use crate::render::{LaneRenderer, HUD};
use crate::scoring::{GameStats};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Loading,
    Playing,
    Paused,
    GameOver,
}

pub struct GameContext {
    pub beatmap: Beatmap,
    pub stats: GameStats,
    pub clock: GameClock,
    pub input: InputHandler,
    pub audio: AudioPlayer,
    pub renderer: LaneRenderer,
    pub hud: HUD,
    pub current_time_ms: u64,
    pub spawned_notes: Vec<usize>,
    pub judged_notes: Vec<usize>,
    pub hit_lane: Option<u8>,
    pub last_judgment: Option<Judgment>,
    pub judgment_timer: u32,
    pub state: GameState,
    pub beatmap_path: String,
}

impl GameContext {
    pub fn new(beatmap: Beatmap, audio_path: &str) -> Result<Self, String> {
        let mut audio = AudioPlayer::new().map_err(|e| e)?;
        let _ = audio.load(audio_path);
        let ctx = GameContext {
            beatmap: beatmap.clone(),
            stats: GameStats::new(),
            clock: GameClock::new(),
            input: InputHandler::new(),
            audio,
            renderer: LaneRenderer::new(4, 20),
            hud: HUD::new(),
            current_time_ms: 0,
            spawned_notes: Vec::new(),
            judged_notes: Vec::new(),
            hit_lane: None,
            last_judgment: None,
            judgment_timer: 0,
            state: GameState::Loading,
            beatmap_path: String::from(""),
        };
        Ok(ctx)
    }

    pub fn update(&mut self) {
        // Very lightweight MVP: advance time and update simple spawn state
        self.current_time_ms = self.clock.elapsed_ms();
        // Transition out of Loading if audio reports as loaded (best-effort)
        if self.state == GameState::Loading {
            if self.audio.is_loaded() {
                self.audio.play();
                self.state = GameState::Playing;
            }
        }
        // Simple simulated input handling per frame (no real game loop here in MVP)
        if let Some(ev) = self.input.poll_event() {
            self.handle_input(ev);
        }
        // Spawn notes based on clock (MVP: simple, 1-second approach)
        let current = self.clock.elapsed_ms();
        for (idx, note) in self.beatmap.notes.iter().enumerate() {
            if self.spawned_notes.contains(&idx) { continue; }
            let spawn_time = note.time_ms.saturating_sub(1000);
            if current >= spawn_time {
                self.spawned_notes.push(idx);
            }
        }
        // End condition: if all notes judged, finish
        if self.judged_notes.len() >= self.beatmap.notes.len() {
            self.state = GameState::GameOver;
        }
    }

    pub fn handle_input(&mut self, event: crate::input::keys::InputEvent) {
        match event {
            crate::input::keys::InputEvent::Hit { lane, .. } => {
                // Find next unjudged note on this lane within hit window
                let mut best_only: Option<(usize, i64, Judgment)> = None;
                for (idx, note) in self.beatmap.notes.iter().enumerate() {
                    if self.judged_notes.contains(&idx) { continue; }
                    if note.lane != lane { continue; }
                    // compute delta between current time and note time
                    let dt = if self.current_time_ms > note.time_ms { self.current_time_ms - note.time_ms } else { note.time_ms - self.current_time_ms };
                    let j = crate::timing::judge(note.time_ms, self.current_time_ms, &crate::timing::HitWindow::default());
                    best_only = Some((idx, dt as i64, j));
                    break;
                }
                if let Some((idx, _dt, j)) = best_only {
                    self.judged_notes.push(idx);
                    self.last_judgment = Some(j.clone());
                    self.stats.add_judgment(&j);
                    // reset hit lane flash
                    self.hit_lane = Some(lane);
                    self.judgment_timer = 30;
                }
            }
            crate::input::keys::InputEvent::Restart => {
                // No-op for integration: we could reset internal state here
                self.state = GameState::Loading;
                self.judged_notes.clear();
                self.spawned_notes.clear();
                self.current_time_ms = 0;
            }
            crate::input::keys::InputEvent::Quit => {
                self.state = GameState::GameOver;
            }
            _ => {}
        }
    }

    pub fn render(&self) -> String {
        // Simple render composition: show HUD and a frame with lanes and notes
        let notes_in_lanes: Vec<(u8, bool)> = self.beatmap.notes.iter()
            .enumerate()
            .filter(|(idx, _)| self.spawned_notes.contains(idx))
            .map(|(idx, n)| (n.lane, matches!(n.kind, NoteKind::Hold)))
            .collect();
        let judgment = self.last_judgment.as_ref().map(|j| match j {
            crate::timing::Judgment::Perfect => "PERFECT!",
            crate::timing::Judgment::Great => "GREAT!",
            crate::timing::Judgment::Good => "GOOD!",
            crate::timing::Judgment::Miss => "MISS!",
        });
        self.renderer.render_frame(&notes_in_lanes, self.hit_lane, judgment)
    }

    pub fn is_game_over(&self) -> bool {
        self.state == GameState::GameOver
    }
}
