use std::time::Duration;

pub struct AdsrEnvelope {
    pub attack: Duration,  // Time for the attack phase ms
    pub decay: Duration,   // Time for the decay phase ms
    pub sustain: f32,      // Sustain level (0.0 to 1.0)
    pub release: Duration, // Time for the release phase ms
}

impl AdsrEnvelope {
    pub fn new(attack: Duration, decay: Duration, sustain: f32, release: Duration) -> AdsrEnvelope {
        // min max sustain level 0.0 to 1.0
        let sustain_min: f32 = 0.0;
        let sustain_max: f32 = 1.0;

        let sustain_limited = sustain_min.max(sustain_max.min(sustain));

        AdsrEnvelope {
            attack,
            decay,
            sustain: sustain_limited,
            release,
        }
    }

    pub fn set_sustain(&mut self, sustain: f32) {
        let sustain_min: f32 = 0.0;
        let sustain_max: f32 = 1.0;

        self.sustain = sustain_min.max(sustain_max.min(sustain));
    }

    // Use times in milliseconds in the amplitude calculation
    pub fn amplitude(&self, t: f32, duration: Duration) -> f32 {
        if t < self.attack.as_secs_f32() {
            t / self.attack.as_secs_f32()
        } else if t < self.attack.as_secs_f32() + self.decay.as_secs_f32() {
            1.0 - (t - self.attack.as_secs_f32()) / self.decay.as_secs_f32() * (1.0 - self.sustain)
        } else if t < duration.as_secs_f32() - self.release.as_secs_f32() {
            self.sustain
        } else if t < duration.as_secs_f32() {
            (1.0 - (t - (duration.as_secs_f32() - self.release.as_secs_f32()))
                / self.release.as_secs_f32())
                * self.sustain
        } else {
            0.0
        }
    }
}
