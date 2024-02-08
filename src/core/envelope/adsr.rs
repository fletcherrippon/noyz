// envelope/adsr.rs

use crate::noyz::Noyz;
use std::time::Duration;

pub struct AdsrEnvelope {
    instance: Noyz,
    pub attack: Duration,         // Time for the attack phase ms
    pub decay: Duration,          // Time for the decay phase ms
    pub sustain: f32,             // Sustain level (0.0 to 1.0)
    pub release: Duration,        // Time for the release phase ms
    pub decay_curve: Option<f32>, // Decay curve shape (-1.0 to 1.0)
}

impl AdsrEnvelope {
    pub fn new(
        instance: Noyz,
        attack: Duration,
        decay: Duration,
        sustain: f32,
        release: Duration,
        decay_curve: Option<f32>,
    ) -> AdsrEnvelope {
        let sustain_limited = sustain.clamp(0.0, 1.0);
        let decay_curve_limited = decay_curve.map(|curve| curve.clamp(-1.0, 1.0));

        AdsrEnvelope {
            instance,
            attack,
            decay,
            sustain: sustain_limited,
            release,
            decay_curve: decay_curve_limited,
        }
    }

    pub fn set_sustain(&mut self, sustain: f32) {
        // Clamping the sustain level between 0.0 to 1.0
        self.sustain = sustain.clamp(0.0, 1.0)
    }

    pub fn set_decay_curve(&mut self, decay_curve: f32) {
        // Clamping the decay curve between -1.0 to 1.0
        self.decay_curve = Some(decay_curve.clamp(0.0, 1.0))
    }

    // This function calculates the amplitude of the envelope at a given time `t`
    pub fn amplitude(&self) -> f32 {
        let t_secs = self.instance.current_time.as_secs_f32();
        let attack_secs = self.attack.as_secs_f32();
        let decay_secs = self.decay.as_secs_f32();
        let release_secs = self.release.as_secs_f32();

        // Use the decay_curve or default to 0.0 for a linear decay
        let decay_curve = self.decay_curve.unwrap_or(0.0);

        if t_secs < attack_secs {
            t_secs / attack_secs
        } else if t_secs < attack_secs + decay_secs {
            let decay_portion = (t_secs - attack_secs) / decay_secs;
            // Apply the decay curve, defaulting to linear if not specified
            1.0 - (decay_portion.powf(decay_curve + 1.0) * (1.0 - self.sustain))
        } else if t_secs < attack_secs + decay_secs + release_secs {
            self.sustain
        } else if t_secs < attack_secs + decay_secs + release_secs + release_secs {
            let release_portion = (t_secs - attack_secs - decay_secs - release_secs) / release_secs;
            self.sustain * (1.0 - release_portion)
        } else {
            0.0
        }
    }
}
