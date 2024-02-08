// noyz.rs

use std::time;

pub struct Noyz {
    pub bpm: u32,
    pub start_time: time::Instant,
    pub current_time: time::Duration,
    pub sample_rate: u32,
    pub volume: f32,
}

impl Noyz {
    pub fn new() -> Noyz {
        Noyz {
            bpm: 120,
            start_time: time::Instant::now(),
            current_time: time::Duration::new(0, 0),
            sample_rate: 44100,
            volume: 1.0,
        }
    }
}
