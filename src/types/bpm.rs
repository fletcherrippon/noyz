use std::time;

pub type Beats = f32;

pub struct BPM {
    pub bpm: Beats,
}

impl BPM {
    pub fn new(bpm: Beats) -> Self {
        BPM { bpm }
    }

    pub fn set_bpm(&mut self, bpm: Beats) {
        self.bpm = bpm;
    }

    pub fn beats_per_second(&self) -> Beats {
        self.bpm / 60.0
    }

    pub fn seconds_per_beat(&self) -> time::Duration {
        let seconds = 1.0 / self.beats_per_second();
        time::Duration::from_secs_f32(seconds)
    }
}
