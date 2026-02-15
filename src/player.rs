// -----------------------------------------------------------------------------
// player.rs — Embedded WAV Playback Utility
// -----------------------------------------------------------------------------
// Plays WAV fragments directly from embedded bytes using include_bytes!().
// No filesystem access. No external Sounds folder. Fully self-contained.
// Computes duration from WAV header for deterministic timing.
// -----------------------------------------------------------------------------

use rodio::{Decoder, OutputStream, Sink};
use rodio::Source;               // <-- REQUIRED for .speed()
use std::io::Cursor;
use std::time::Duration;
use std::thread;
use hound::WavReader;

// -----------------------------------------------------------------------------
// Speech rate multiplier (1.0 = normal, >1.0 = faster)
// -----------------------------------------------------------------------------
const SPEECH_RATE: f32 = 1.0;    // <-- You can tune this anytime

// -----------------------------------------------------------------------------
// Compute WAV duration from embedded bytes
// -----------------------------------------------------------------------------
fn wav_duration_ms(bytes: &[u8]) -> u64 {
    let reader = WavReader::new(Cursor::new(bytes))
        .expect("Invalid WAV data");
    let spec = reader.spec();

    let data_chunk_size = bytes.len() as u64 - 44;
    let bytes_per_sample = (spec.bits_per_sample / 8) as u64;
    let bytes_per_frame = bytes_per_sample * spec.channels as u64;
    let total_frames = data_chunk_size / bytes_per_frame;

    (total_frames * 1000) / spec.sample_rate as u64
}

// -----------------------------------------------------------------------------
// Play embedded WAV bytes (blocking)
// -----------------------------------------------------------------------------
pub fn play_wav_bytes(bytes: &'static [u8]) {
    let duration_ms = wav_duration_ms(bytes);

    let (_stream, handle) =
        OutputStream::try_default().expect("No audio output device available");

    let sink = Sink::try_new(&handle)
        .expect("Failed to create audio sink");

    let cursor = Cursor::new(bytes.to_vec());

    let source = Decoder::new(cursor)
        .expect("Could not decode WAV data")
        .speed(SPEECH_RATE);      // <-- Speech rate applied here

    sink.append(source);

    thread::sleep(Duration::from_millis(duration_ms));
}
