pub struct Channel {
    pub samples: Vec<i16>,
    pub volume: f32,
    pub gain: f32,
    pub filter_value: f32, // -1 to 1 for LPF to HPF
}

impl Channel {
    fn new(volume: f32) -> Self {
        Channel {
            samples: Vec::new(),
            volume,
            gain: 1.0,
            filter_value: 0.0,
        }
    }

    fn set_samples(&mut self, new_samples: Vec<i16>) {
        self.samples = new_samples;
    }

    fn clear_samples(&mut self) {
        self.samples.clear();
    }

    fn filter(&mut self) {
        let mut filtered_samples: Vec<i16> = vec![0; self.samples.len()];
        let mut previous_sample: f32 = 0.0;

        let high_pass_coefficient = self.filter_value.max(0.0); // 0 to 1
        let low_pass_coefficient = (-self.filter_value).max(0.0); // 0 to 1

        for (i, &sample) in self.samples.iter().enumerate() {
            let current_sample = sample as f32;

            // Simple one-pole low-pass filter
            let lpf = (previous_sample * low_pass_coefficient)
                + (current_sample * (1.0 - low_pass_coefficient));

            // Simple one-pole high-pass filter
            let hpf = (current_sample - previous_sample) * high_pass_coefficient
                + (previous_sample * (1.0 - high_pass_coefficient));

            // Choose which filter to apply based on the filter_value
            let filtered_sample = if self.filter_value > 0.0 {
                hpf
            } else if self.filter_value < 0.0 {
                lpf
            } else {
                current_sample
            };

            filtered_samples[i] = filtered_sample as i16;
            previous_sample = filtered_sample; // Store the filtered sample for the next iteration
        }

        self.samples = filtered_samples; // Update the samples with the filtered output
    }

    fn modulate_filter<F>(&mut self, modulation_fn: F)
    where
        F: Fn(f32) -> f32,
    {
        let base_filter_value = self.filter_value; // Home base for filter modulation
        let mut filtered_samples: Vec<i16> = vec![0; self.samples.len()];
        let mut previous_sample: f32 = 0.0;

        for (i, &sample) in self.samples.iter().enumerate() {
            let t = i as f32 / self.samples.len() as f32;
            let modulation_value = modulation_fn(t);
            let modulated_filter_value = base_filter_value + modulation_value;

            let high_pass_coefficient = modulated_filter_value.max(0.0); // 0 to 1
            let low_pass_coefficient = (-modulated_filter_value).max(0.0); // 0 to 1

            let current_sample = sample as f32;

            // Simple one-pole low-pass filter
            let lpf = (previous_sample * low_pass_coefficient)
                + (current_sample * (1.0 - low_pass_coefficient));

            // Simple one-pole high-pass filter
            let hpf = (current_sample - previous_sample) * high_pass_coefficient
                + (previous_sample * (1.0 - high_pass_coefficient));

            // Choose which filter to apply based on the modulated filter value
            let filtered_sample = if modulated_filter_value > 0.0 {
                hpf
            } else if modulated_filter_value < 0.0 {
                lpf
            } else {
                current_sample
            };

            filtered_samples[i] = filtered_sample as i16;
            previous_sample = filtered_sample; // Store the filtered sample for the next iteration
        }

        self.samples = filtered_samples; // Update the samples with the modulated filtered output
    }

    fn set_filter_value(&mut self, value: f32) {
        // Ensure the value is clamped between -1.0 and 1.0
        self.filter_value = value.max(-1.0).min(1.0);
    }
}
