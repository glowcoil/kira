use crate::{
	audio_stream::{AudioStream, AudioStreamId},
	command::StreamCommand,
	manager::TrackIndex,
};

use super::mixer::Mixer;

use basedrop::Owned;
use indexmap::IndexMap;

pub(crate) struct Streams {
	streams: IndexMap<AudioStreamId, (TrackIndex, Owned<Box<dyn AudioStream>>)>,
}

impl Streams {
	pub fn new(capacity: usize) -> Self {
		Self {
			streams: IndexMap::with_capacity(capacity),
		}
	}

	pub fn run_command(&mut self, command: StreamCommand) {
		match command {
			StreamCommand::AddStream(stream_id, track_id, stream) => {
				self.streams.insert(stream_id, (track_id, stream));
			}
			StreamCommand::RemoveStream(stream_id) => {
				self.streams.remove(&stream_id);
			}
		}
	}

	pub fn process(&mut self, dt: f64, mixer: &mut Mixer) {
		for (track, stream) in self.streams.values_mut() {
			mixer.add_input(*track, stream.next(dt));
		}
	}
}
