use std::f32::consts::PI;
use std::time::Duration;

pub struct Oscillator {
    pub sample_rate: u32,
    pub frequency: f32,
    pub amplitude: f32,
    pub duration: Duration,
    pub samples: Vec<i16>,
}

impl Oscillator {
    pub fn new(sample_rate: u32, frequency: f32, duration: Duration, amplitude: f32) -> Oscillator {
        Oscillator {
            sample_rate,
            frequency,
            duration,
            amplitude,
            samples: Vec::new(),
        }
    }

    fn create_sine_wave(&self, t: f32, f: f32) -> f32 {
        (f * t * 2.0 * PI).sin()
    }

    fn create_square_wave(&self, t: f32, f: f32) -> f32 {
        if (f * t * 2.0 * PI).sin() >= 0.0 {
            1.0
        } else {
            -1.0
        }
    }

    fn create_triangle_wave(&self, t: f32, f: f32) -> f32 {
        2.0 * (2.0 * (t / f - (t / f + 0.5).floor())).abs() - 1.0
    }

    fn create_sawtooth_wave(&self, t: f32, f: f32) -> f32 {
        let phase = t * f % 1.0; // Phase accumulator modulo 1.0
        2.0 * phase - 1.0 // Scale to range [-1, 1]
    }

    fn create_wave<F>(&self, oscillator_op: F) -> Vec<i16>
    where
        F: Fn(f32, f32) -> f32,
    {
        let total_samples = (self.sample_rate as f32 * self.duration.as_secs_f32()) as usize;
        let mut samples: Vec<i16> = Vec::with_capacity(total_samples);

        for t in (0..total_samples).map(|x| x as f32 / self.sample_rate as f32) {
            let v = oscillator_op(t, self.frequency);
            let scaled_value = v * self.amplitude * 32767.0;
            samples.push(scaled_value as i16);
        }

        samples
    }

    #[allow(dead_code)]
    pub fn sine_wave(&mut self) -> Vec<i16> {
        self.samples = self.create_wave(|t, f| self.create_sine_wave(t, f));
        self.samples.clone()
    }

    #[allow(dead_code)]
    pub fn square_wave(&mut self) -> Vec<i16> {
        self.samples = self.create_wave(|t, f| self.create_square_wave(t, f));
        self.samples.clone()
    }

    #[allow(dead_code)]
    pub fn triangle_wave(&mut self) -> Vec<i16> {
        self.samples = self.create_wave(|t, f| self.create_triangle_wave(t, f));
        self.samples.clone()
    }

    #[allow(dead_code)]
    pub fn sawtooth_wave(&mut self) -> Vec<i16> {
        self.samples = self.create_wave(|t, f| self.create_sawtooth_wave(t, f));
        self.samples.clone()
    }

    pub fn fm<F>(&mut self, modulation_fn: F)
    where
        F: Fn(f32) -> f32,
    {
        let mut phase = 0.0;
        let phase_step = self.frequency * 2.0 * PI / self.sample_rate as f32;

        for (i, sample) in self.samples.iter_mut().enumerate() {
            let t = i as f32 / self.sample_rate as f32;
            // Call the modulation function for the current time t
            let modulation_value = modulation_fn(t);
            // Calculate the new phase with modulation applied
            phase += phase_step + modulation_value * phase_step;
            // Wrap the phase to keep it within the 0 to 2*PI range
            while phase > 2.0 * PI {
                phase -= 2.0 * PI;
            }
            // Apply FM by modifying the sample based on the new phase
            *sample = (phase.sin() * self.amplitude * 32767.0) as i16;
        }
    }

    pub fn am<F>(&mut self, modulation_fn: F)
    where
        F: Fn(f32) -> f32,
    {
        for (i, sample) in self.samples.iter_mut().enumerate() {
            let t = i as f32 / self.sample_rate as f32;
            // Call the modulation function for the current time t
            let modulation_value = modulation_fn(t);
            // Apply the modulation value directly to the sample
            *sample = (*sample as f32 * modulation_value) as i16;
        }
    }
}
