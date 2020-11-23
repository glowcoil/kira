mod clip;

pub use clip::SoundClip;

use std::{
	hash::Hash,
	sync::atomic::{AtomicUsize, Ordering},
};

use indexmap::IndexMap;

use crate::{
	sound::{Sound, SoundId},
	Frame,
};

static NEXT_ARRANGEMENT_INDEX: AtomicUsize = AtomicUsize::new(0);

/**
A unique identifier for an [arrangement](Arrangement).

You cannot create this manually - an arrangement ID is created
when you create a arrangement with an [`AudioManager`](crate::manager::AudioManager).
*/
#[derive(Debug, Copy, Clone)]
pub struct ArrangementId {
	index: usize,
	duration: f64,
}

impl ArrangementId {
	pub(crate) fn new(duration: f64) -> Self {
		let index = NEXT_ARRANGEMENT_INDEX.fetch_add(1, Ordering::Relaxed);
		Self { index, duration }
	}

	pub fn duration(&self) -> f64 {
		self.duration
	}
}

impl PartialEq for ArrangementId {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}

impl Eq for ArrangementId {}

impl Hash for ArrangementId {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.index.hash(state);
	}
}

#[derive(Debug, Clone)]
pub struct Arrangement {
	clips: Vec<SoundClip>,
	duration: f64,
}

impl Arrangement {
	pub fn new() -> Self {
		Self {
			clips: vec![],
			duration: 0.0,
		}
	}

	pub fn add_clip(mut self, clip: SoundClip) -> Self {
		self.duration = self.duration.max(clip.clip_time_range.end);
		self.clips.push(clip);
		self
	}

	pub fn duration(&self) -> f64 {
		self.duration
	}

	pub(crate) fn get_frame_at_position(
		&self,
		position: f64,
		sounds: &IndexMap<SoundId, Sound>,
	) -> Frame {
		let mut frame = Frame::from_mono(0.0);
		for clip in &self.clips {
			frame += clip.get_frame_at_position(position, sounds);
		}
		frame
	}
}

impl From<SoundId> for Arrangement {
	fn from(id: SoundId) -> Self {
		Self::new().add_clip(SoundClip::new(id, 0.0))
	}
}
