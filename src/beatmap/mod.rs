// Beatmap-related modules
pub mod types;
pub mod parser;

pub use self::types::{Beatmap, Note, NoteKind, TimingPoint};

#[cfg(test)]
mod tests;
