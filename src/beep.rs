use rodio::{OutputStream, source::SineWave};
use std::time::Duration;
use std::thread;
pub fn play_beep(duration: Duration, frequency: f32) {
    // Crea uno stream di output per riprodurre suoni
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Crea una sorgente sonora con una sinusoide (tono)
    let source = SineWave::new(frequency);

    // Riproduce il suono
    let _ = stream_handle.play_raw(source);

    thread::sleep(duration);
}