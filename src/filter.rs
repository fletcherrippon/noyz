impl Filter {
    pub fn new() -> Self {
        Filter { filter_value: 0.0 }
    }

    // Implement the basic filter application logic
    pub fn set(&self, samples: &mut [i16]) {
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

    // Implement the modulation logic
    pub fn modulate<F>(&self, samples: &mut [i16], modulation_fn: F)
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
}

pub trait Filterable {
    fn get_filter(&mut self) -> &mut Filter;

    fn apply_filter(&mut self) {
        self.get_filter().apply(&mut self.get_samples());
    }

    fn modulate_filter<F>(&mut self, modulation_fn: F)
    where
        F: Fn(f32) -> f32,
    {
        self.get_filter()
            .modulate(&mut self.get_samples(), modulation_fn);
    }

    // This method must be implemented by the struct to return its sample buffer
    fn get_samples(&mut self) -> &mut [i16];
}
