use crate::types::rhythm::tempo::bpm::beat::Beat;
use std::time;

pub struct BPM {
    pub bpm: Beat,
}

impl Default for BPM {
    fn default() -> Self {
        BPM { bpm: 120.0 }
    }
}

impl BPM {
    pub fn new(bpm: Beat) -> Self {
        BPM { bpm }
    }

    pub fn set(&mut self, bpm: Beat) {
        self.bpm = bpm;
    }
}
