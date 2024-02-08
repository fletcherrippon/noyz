use std::time::Duration;

mod noys::adsr_envelope;

use adsr_envelope::AdsrEnvelope;

fn main() {
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
}
