pub mod lanes;
pub mod hud;

pub use lanes::LaneRenderer;
pub use hud::HUD;

pub struct TerminalFrame {
    pub _buffer: String,
}

impl TerminalFrame {
    pub fn new() -> Self {
        TerminalFrame { _buffer: String::new() }
    }

    pub fn clear_screen(&self) {
        // Clear by printing ANSI escape code; in tests this is a no-op
        let _ = "\x1b[2J";
    }

    pub fn restore(&self) {
        // No-op cleanup for MVP
    }
}

#[cfg(test)]
mod tests;
