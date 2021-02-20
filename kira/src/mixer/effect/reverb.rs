use std::ops::Range;

use crate::{util::lerp, Frame};

use super::{
	delay::{Delay, DelaySettings},
	filter::{FilterMode, FilterSettings},
	Effect,
};

#[derive(Debug)]
pub struct Reverb {
	delays: Vec<Delay>,
}

impl Reverb {
	pub fn new() -> Self {
		Self {
			delays: vec![
				Delay::new(
					DelaySettings::new()
						.delay_time(0.02)
						.feedback(-0.5)
						.filter_settings(
							FilterSettings::new()
								.mode(FilterMode::BandPass)
								.cutoff(4000.0),
						),
				),
				Delay::new(
					DelaySettings::new()
						.delay_time(0.03)
						.feedback(-0.5)
						.filter_settings(
							FilterSettings::new()
								.mode(FilterMode::BandPass)
								.cutoff(4000.0),
						),
				),
				Delay::new(
					DelaySettings::new()
						.delay_time(0.05)
						.feedback(-0.5)
						.filter_settings(
							FilterSettings::new()
								.mode(FilterMode::BandPass)
								.cutoff(4000.0),
						),
				),
				Delay::new(
					DelaySettings::new()
						.delay_time(0.07)
						.feedback(-0.5)
						.filter_settings(
							FilterSettings::new()
								.mode(FilterMode::BandPass)
								.cutoff(4000.0),
						),
				),
				Delay::new(
					DelaySettings::new()
						.delay_time(0.011)
						.feedback(-0.5)
						.filter_settings(
							FilterSettings::new()
								.mode(FilterMode::BandPass)
								.cutoff(4000.0),
						),
				),
				Delay::new(
					DelaySettings::new()
						.delay_time(0.013)
						.feedback(-0.5)
						.filter_settings(
							FilterSettings::new()
								.mode(FilterMode::BandPass)
								.cutoff(4000.0),
						),
				),
				Delay::new(
					DelaySettings::new()
						.delay_time(0.017)
						.feedback(-0.5)
						.filter_settings(
							FilterSettings::new()
								.mode(FilterMode::BandPass)
								.cutoff(4000.0),
						),
				),
				Delay::new(
					DelaySettings::new()
						.delay_time(0.023)
						.feedback(-0.5)
						.filter_settings(
							FilterSettings::new()
								.mode(FilterMode::BandPass)
								.cutoff(4000.0),
						),
				),
				Delay::new(
					DelaySettings::new()
						.delay_time(0.029)
						.feedback(-0.5)
						.filter_settings(
							FilterSettings::new()
								.mode(FilterMode::BandPass)
								.cutoff(4000.0),
						),
				),
			],
		}
	}
}

impl Effect for Reverb {
	fn process(
		&mut self,
		dt: f64,
		input: crate::Frame,
		parameters: &crate::parameter::Parameters,
	) -> crate::Frame {
		let mut output = Frame::from_mono(0.0);
		for delay in &mut self.delays {
			output += delay.process(dt, input, parameters);
		}
		output
	}
}
