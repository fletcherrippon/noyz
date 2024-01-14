mod adsr_envelope;
mod oscillator;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::sync::Arc;
use std::time::Duration;

use adsr_envelope::AdsrEnvelope;
use oscillator::Oscillator;

const SAMPLE_RATE: u32 = 44100; // Sample rate in Hz
const TONE_HZ: f32 = 60.0; // Frequency of A4 note
const VOLUME: f32 = 1.0; // Volume (0.0 to 1.0)
const DURATION: u32 = 800; // Duration in ms

fn write_wav_header(
    writer: &mut BufWriter<File>,
    sample_rate: u32,
    duration: u32,
) -> std::io::Result<()> {
    let byte_rate = sample_rate * 2; // 16 bit mono, 2 bytes per sample
    let block_align = 2u16; // 16 bit mono, 2 bytes per sample
    let sub_chunk_2_size = sample_rate * duration * 2; // 16 bit mono
    let chunk_size = 36 + sub_chunk_2_size;

    writer.write_all(b"RIFF")?;
    writer.write_all(&(chunk_size as u32).to_le_bytes())?;
    writer.write_all(b"WAVE")?;
    writer.write_all(b"fmt ")?;
    writer.write_all(&(16u32).to_le_bytes())?; // PCM
    writer.write_all(&(1u16).to_le_bytes())?; // PCM format
    writer.write_all(&(1u16).to_le_bytes())?; // Mono
    writer.write_all(&sample_rate.to_le_bytes())?;
    writer.write_all(&(byte_rate as u32).to_le_bytes())?;
    writer.write_all(&block_align.to_le_bytes())?;
    writer.write_all(&(16u16).to_le_bytes())?; // 16 bit
    writer.write_all(b"data")?;
    writer.write_all(&(sub_chunk_2_size as u32).to_le_bytes())?;

    Ok(())
}

fn save_to_wav_file(samples: Vec<i16>) -> std::io::Result<()> {
    let file_naem = format!("sine_wave_sr-{}_hz-{}.wav", SAMPLE_RATE, TONE_HZ);
    let file = File::create(file_naem)?;
    let mut writer = BufWriter::new(file);

    // Corrected call to write_wav_header
    write_wav_header(&mut writer, SAMPLE_RATE, 1)?;

    // Write audio data
    for sample in samples {
        writer.write_all(&sample.to_le_bytes())?;
    }

    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Error> {
    // Get the default host
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or(Error::new(
        std::io::ErrorKind::NotFound,
        "No output device available",
    ))?;
    // Get device config for the default output device
    let config = device
        .default_output_config()
        .map_err(|e| Error::new(std::io::ErrorKind::Other, e))?;
    // Get the sample format for the config
    let sample_format = config.sample_format();
    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;
    // create a sine wave oscillator
    let mut o = Oscillator::new(
        sample_rate,
        TONE_HZ,
        Duration::from_millis(DURATION as u64),
        VOLUME,
    );

    o.square_wave();

    // Create an ADSR envelope for amplitude modulation (AM)
    let am_envelope = AdsrEnvelope::new(
        Duration::from_millis(2),   // Short attack for punch
        Duration::from_millis(0),   // Short decay
        0.52,                       // Low sustain for a quick fade-out
        Duration::from_millis(222), // Short release
    );

    // Create an ADSR envelope for frequency modulation (FM)
    let fm_envelope = AdsrEnvelope::new(
        Duration::from_millis(2),  // Short attack
        Duration::from_millis(72), // Shorter decay to quickly drop the pitch
        0.0,                       // Zero sustain to avoid prolonged frequency change
        Duration::from_millis(0),  // Short release
    );

    o.fm(|t| fm_envelope.amplitude(t, Duration::from_millis(132)) * 2.5);
    o.am(|t| am_envelope.amplitude(t, Duration::from_millis(320)));

    let sine_wave_samples = Arc::new(o.samples);
    let sine_wave_samples_clone = Arc::clone(&sine_wave_samples);
    let mut sample_index = 0;

    let err_fn = |err| eprintln!("An error occurred on the audio stream: {}", err);

    // Build a stream with the sample format from the config
    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    if sample_index < sine_wave_samples_clone.len() {
                        let sample =
                            sine_wave_samples_clone[sample_index] as f32 / std::i16::MAX as f32;
                        for out in frame.iter_mut() {
                            *out = sample;
                        }
                        sample_index = (sample_index + 1) % sine_wave_samples_clone.len();
                    }
                }
            },
            err_fn,
            None,
        )
        .map_err(|e| Error::new(std::io::ErrorKind::Other, e))?;

    stream
        .play()
        .map_err(|e| Error::new(std::io::ErrorKind::Other, e))?;

    std::thread::sleep(std::time::Duration::from_millis(DURATION as u64)); // Keep the stream alive for the duration

    save_to_wav_file(sine_wave_samples.to_vec())?; // Save the generated sine wave to a WAV file

    Ok(())
}
