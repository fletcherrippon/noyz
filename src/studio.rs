// noyz.rs
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::time;

use super::types::{
    bpm::{Beats, BPM},
    time_signature::TimeSignatures,
};

struct Loop {
    start: [u32; 2],
    end: [u32; 2],
    active: bool,
}

pub struct Config {
    pub bpm: Beats,
    pub time_signature: TimeSignatures, // You might want to create a more specific type here
}

pub struct Studio {
    pub bpm: BPM,
    pub start_time: time::Instant,
    pub current_time: time::Duration,
    pub sample_rate: u32,
    pub volume: f32,
}

impl Studio {
    pub fn new(config: Config) -> Studio {
        Studio {
            bpm: BPM::new(config.bpm),
            start_time: time::Instant::now(),
            current_time: time::Duration::new(0, 0),
            sample_rate: 44100,
            volume: 1.0,
        }
    }

    pub fn set_bpm(&mut self, bpm: Beats) {
        self.bpm.set_bpm(bpm);
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    pub fn update_time(&mut self) {
        self.current_time = self.start_time.elapsed();
    }

    pub fn reset_time(&mut self) {
        self.start_time = time::Instant::now();
        self.current_time = time::Duration::new(0, 0);
    }

    pub fn time_to_beats(&self, time: time::Duration) -> f32 {
        let seconds = time.as_secs_f32();
        let beats = seconds / 60.0 / self.bpm.bpm;
        beats
    }

    pub fn beats_to_time(&self, beats: f32) -> time::Duration {
        let seconds = beats * 60.0 * self.bpm.bpm;
        time::Duration::from_secs_f32(seconds)
    }
}
