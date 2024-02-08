use noyz::{
    studio::{Config, Studio},
    types::time_signature::TimeSignatures,
};

fn main() {
    let config = Config {
        bpm: 120.0,
        time_signature: TimeSignatures::FourFour,
    };

    let noyz = Studio::new(config);

    println!("BPM: {}", noyz.bpm.seconds_per_beat().as_secs_f32());
}
