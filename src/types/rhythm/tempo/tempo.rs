use crate::types::rhythm::tempo::bpm::bpm::BPM;
use crate::types::rhythm::time_signature::time_signature::TimeSignatures;

use std::time::Duration;

use super::bpm::beat::Beat;

pub struct Tempo {
    pub bpm: BPM,
    pub time_signature: TimeSignatures,
}

impl Default for Tempo {
    fn default() -> Self {
        Tempo {
            bpm: Default::default(),
            time_signature: TimeSignatures::default(),
        }
    }
}

impl Tempo {
    pub fn new<B: Into<Option<Beat>>, TS: Into<Option<TimeSignatures>>>(
        bpm: B,
        time_signature: TS,
    ) -> Self {
        match (bpm.into(), time_signature.into()) {
            (Some(bpm), Some(time_signature)) => Tempo {
                bpm: BPM::new(bpm),
                time_signature,
            },
            (Some(bpm), None) => Tempo {
                bpm: BPM::new(bpm),
                time_signature: TimeSignatures::default(),
            },
            (None, Some(time_signature)) => Tempo {
                bpm: Default::default(),
                time_signature: time_signature,
            },
            (None, None) => Tempo::default(),
        }
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm.set(bpm);
    }

    pub fn set_time_signature(&mut self, time_signature: TimeSignatures) {
        self.time_signature = time_signature;
    }

    pub fn beats_over_time(&self, time: Duration) -> Beat {
        let seconds = time.as_secs_f32();
        let beats_per_second = self.bpm.bpm / 60.0;

        seconds / beats_per_second
    }

    pub fn beats_to_time(&self, beats: Beat) -> Duration {
        let seconds = beats * 60.0 / self.bpm.bpm;
        Duration::from_secs_f32(seconds)
    }

    pub fn bars_to_beats(&self, bars: f32) -> f32 {
        bars * self.time_signature.beats() as f32
    }

    pub fn bars_to_time(&self, bars: f32) -> Duration {
        let beats = self.bars_to_beats(bars);
        self.beats_to_time(beats)
    }
}
