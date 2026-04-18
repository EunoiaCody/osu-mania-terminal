use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Duration;

use crossterm::event::{KeyCode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    Hit { lane: u8, timestamp_ms: u64 },
    Quit,
    Pause,
    Resume,
    Restart,
}

// Hardcoded key map for MVP: D/F/J/K to lanes 0-3
pub fn map_char_to_lane(ch: char, lanes: u8) -> Option<u8> {
    match ch {
        'd' | 'D' => Some(0.min((lanes - 1) as u8)),
        'f' | 'F' => Some(1.min((lanes - 1) as u8)),
        'j' | 'J' => Some(2.min((lanes - 1) as u8)),
        'k' | 'K' => Some(3.min((lanes - 1) as u8)),
        _ => None,
    }
}

// Determine InputEvent from a KeyCode (used for unit tests)
pub fn map_keycode_to_event(code: KeyCode, paused: &mut bool) -> Option<InputEvent> {
    match code {
        KeyCode::Char('d') | KeyCode::Char('D') => Some(InputEvent::Hit { lane: 0, timestamp_ms: current_time_ms() }),
        KeyCode::Char('f') | KeyCode::Char('F') => Some(InputEvent::Hit { lane: 1, timestamp_ms: current_time_ms() }),
        KeyCode::Char('j') | KeyCode::Char('J') => Some(InputEvent::Hit { lane: 2, timestamp_ms: current_time_ms() }),
        KeyCode::Char('k') | KeyCode::Char('K') => Some(InputEvent::Hit { lane: 3, timestamp_ms: current_time_ms() }),
        KeyCode::Esc => Some(InputEvent::Quit),
        KeyCode::Enter => {
            *paused = !*paused;
            if *paused { Some(InputEvent::Pause) } else { Some(InputEvent::Resume) }
        }
        KeyCode::Char('r') | KeyCode::Char('R') => Some(InputEvent::Restart),
        _ => None,
    }
}

// Non-blocking current time in milliseconds since epoch
fn current_time_ms() -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0));
    now.as_millis() as u64
}

pub struct InputHandler {
    paused: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        // Enable raw mode, best-effort; ignore failures in environments without TTY
        let _ = crossterm::terminal::enable_raw_mode();
        InputHandler { paused: false }
    }

    pub fn poll_event(&mut self) -> Option<InputEvent> {
        use crossterm::event::{poll, read, Event};
        // Non-blocking poll with zero timeout
        if poll(Duration::from_millis(0)).unwrap_or(false) {
            if let Ok(Event::Key(key)) = read() {
                return map_keycode_to_event(key.code, &mut self.paused);
            }
        }
        None
    }

    pub fn cleanup(&self) {
        let _ = crossterm::terminal::disable_raw_mode();
    }
}
