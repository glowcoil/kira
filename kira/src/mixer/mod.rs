//! Organizes and applies effects to audio.

pub mod effect;
pub(crate) mod effect_slot;
mod track;

pub(crate) use track::Track;
pub use track::{handle, SubTrackId, TrackIndex, TrackSettings};
