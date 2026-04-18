use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct Beatmap {
    pub title: String,
    pub artist: String,
    pub bpm: f64,
    pub lanes: u8,
    pub notes: Vec<Note>,
    pub timing_points: Vec<TimingPoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NoteKind {
    Note,
    Hold,
}

#[derive(Debug, Clone)]
pub struct Note {
    pub time_ms: u64,
    pub lane: u8,
    pub kind: NoteKind,
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct TimingPoint {
    pub time_ms: f64,
    pub beat_length_ms: f64,
    pub uninherited: bool,
}
