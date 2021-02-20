//! An interface for controlling instances of sounds and arrangements.

use std::{
	sync::{Arc, Mutex},
	time::Instant,
};

use atomic::{Atomic, Ordering};
use ringbuf::{Consumer, Producer};

use crate::{
	command::{
		producer::{CommandError, CommandProducer},
		InstanceCommand,
	},
	Value,
};

use super::{
	InstanceId, InstanceState, PauseInstanceSettings, ResumeInstanceSettings, StopInstanceSettings,
};

// TODO: add a manual debug impl
#[derive(Clone)]
/// Allows you to control an instance of a sound or arrangement.
pub struct InstanceHandle {
	id: InstanceId,
	state: Arc<Atomic<InstanceState>>,
	command_producer: CommandProducer,
	playback_position_request_producer: Arc<Mutex<Producer<()>>>,
	playback_position_consumer: Arc<Mutex<Consumer<f64>>>,
}

impl InstanceHandle {
	pub(crate) fn new(
		id: InstanceId,
		state: Arc<Atomic<InstanceState>>,
		command_producer: CommandProducer,
		playback_position_request_producer: Producer<()>,
		playback_position_consumer: Consumer<f64>,
	) -> Self {
		Self {
			id,
			state,
			command_producer,
			playback_position_request_producer: Arc::new(Mutex::new(
				playback_position_request_producer,
			)),
			playback_position_consumer: Arc::new(Mutex::new(playback_position_consumer)),
		}
	}

	/// Returns the ID of the instance.
	pub fn id(&self) -> InstanceId {
		self.id
	}

	/// Returns the playback state of the instance.
	pub fn state(&self) -> InstanceState {
		self.state.load(Ordering::Relaxed)
	}

	pub fn playback_position(&mut self) -> f64 {
		let start_time = Instant::now();
		self.playback_position_request_producer
			.lock()
			.unwrap()
			.push(())
			.ok();
		let mut consumer = self.playback_position_consumer.lock().unwrap();
		let position = loop {
			if let Some(position) = consumer.pop() {
				break position;
			}
		};
		println!(
			"roundtrip time: {} us",
			(Instant::now() - start_time).as_micros()
		);
		position
	}

	/// Sets the volume of the instance.
	pub fn set_volume(&mut self, volume: impl Into<Value<f64>>) -> Result<(), CommandError> {
		self.command_producer
			.push(InstanceCommand::SetInstanceVolume(self.id, volume.into()).into())
	}

	/// Sets the pitch of the instance.
	pub fn set_pitch(&mut self, pitch: impl Into<Value<f64>>) -> Result<(), CommandError> {
		self.command_producer
			.push(InstanceCommand::SetInstancePitch(self.id, pitch.into()).into())
	}

	/// Sets the panning of the instance.
	pub fn set_panning(&mut self, panning: impl Into<Value<f64>>) -> Result<(), CommandError> {
		self.command_producer
			.push(InstanceCommand::SetInstancePanning(self.id, panning.into()).into())
	}

	/// Offsets the playback position of the instance by the specified amount (in seconds).
	pub fn seek(&mut self, offset: f64) -> Result<(), CommandError> {
		self.command_producer
			.push(InstanceCommand::SeekInstance(self.id, offset).into())
	}

	/// Sets the playback position of the instance to the specified time (in seconds).
	pub fn seek_to(&mut self, position: f64) -> Result<(), CommandError> {
		self.command_producer
			.push(InstanceCommand::SeekInstanceTo(self.id, position).into())
	}

	/// Pauses the instance.
	pub fn pause(&mut self, settings: PauseInstanceSettings) -> Result<(), CommandError> {
		self.command_producer
			.push(InstanceCommand::PauseInstance(self.id, settings).into())
	}

	/// Resumes the instance.
	pub fn resume(&mut self, settings: ResumeInstanceSettings) -> Result<(), CommandError> {
		self.command_producer
			.push(InstanceCommand::ResumeInstance(self.id, settings).into())
	}

	/// Stops the instance.
	pub fn stop(&mut self, settings: StopInstanceSettings) -> Result<(), CommandError> {
		self.command_producer
			.push(InstanceCommand::StopInstance(self.id, settings).into())
	}
}
