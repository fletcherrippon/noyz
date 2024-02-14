// noyz.rs
use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, Host,
};
use std::time;

use super::types::rhythm::tempo::tempo::Tempo;

struct Loop {
    start: [u32; 2],
    end: [u32; 2],
    active: bool,
}

pub struct Audio {
    pub host: Host,
    pub input: Device,
    pub output: Device,
}

impl Default for Audio {
    fn default() -> Self {
        let host = cpal::default_host();
        let input = host.default_input_device().unwrap();
        let output = host.default_output_device().unwrap();

        Audio {
            host,
            input,
            output,
        }
    }
}

impl Audio {
    pub fn new<I: Into<Option<Device>>, O: Into<Option<Device>>>(input: I, output: O) -> Self {
        let host = cpal::default_host();
        let input = match input.into() {
            Some(input) => input,
            None => host.default_input_device().unwrap(),
        };
        let output = match output.into() {
            Some(output) => output,
            None => host.default_output_device().unwrap(),
        };

        Audio {
            host,
            input,
            output,
        }
    }

    pub fn list_drivers(&self) {
        let hosts = cpal::available_hosts();

        println!("Available Drivers:");
        for host_id in hosts {
            // The `name` method is called on `HostId`, not on `Host`
            println!("\t{}", host_id.name());
        }
    }

    pub fn list_input_devices(&self) {
        let inputs = self.host.input_devices().unwrap();

        println!("Input Devices: ");
        for input in inputs {
            // print input devices
            println!("\t{}", input.name().unwrap());
        }
    }

    pub fn list_output_devices(&self) {
        let outputs = self.host.output_devices().unwrap();

        println!("Output Devices: ");
        for output in outputs {
            // print output devices
            println!("\t{}", output.name().unwrap());
        }
    }

    pub fn list_devices(&self) {
        self.list_input_devices();
        self.list_output_devices();
    }
}

pub struct Config {
    pub tempo: Tempo,
    pub audio: Audio,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            tempo: Tempo::default(),
            audio: Audio::default(),
        }
    }
}

pub struct Studio {
    pub tempo: Tempo,
    pub audio: Audio,
    pub start_time: time::Instant,
    pub current_time: time::Duration,
    pub volume: f32,
}

impl Default for Studio {
    fn default() -> Self {
        Studio {
            tempo: Tempo::default(),
            audio: Audio::default(),
            start_time: time::Instant::now(),
            current_time: time::Duration::new(0, 0),
            volume: 1.0,
        }
    }
}

impl Studio {
    pub fn new<C: Into<Option<Config>>>(config: C) -> Studio {
        match config.into() {
            Some(config) => Studio {
                tempo: config.tempo,
                audio: config.audio,
                start_time: time::Instant::now(),
                current_time: time::Duration::new(0, 0),
                volume: 1.0,
            },
            None => Studio::default(),
        }
    }

    // Constructor that optionally takes a configuration
    pub fn with_default_config() -> Studio {
        Studio::new(None)
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
}
