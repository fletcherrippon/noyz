use noyz::{
    studio::{Config, Studio},
    types::rhythm::{tempo::tempo::Tempo, time_signature::time_signature::TimeSignatures},
};

fn main() {
    let config = Config {
        tempo: Tempo::new(120.0, TimeSignatures::ThreeFour),
        ..Default::default()
    };

    let noyz = Studio::new(config);

    println!("BPM: {}", noyz.tempo.bars_to_time(1.0).as_secs_f32());

    noyz.audio.list_drivers();
    noyz.audio.list_devices();
}
