use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn get_audio_level() -> Arc<Mutex<f32>> {
    let audio_level = Arc::new(Mutex::new(0.0));
    let audio_clone = audio_level.clone();
    thread::spawn(move || {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("Failed to get default input device");
        let config = device
            .default_input_config()
            .expect("Failed to get default config")
            .config();

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        let stream = device
            .build_input_stream(
                &config,
                move |data: &[f32], _| {
                    // Compute RMS level
                    let sum_squares: f32 = data.iter().map(|&sample| sample * sample).sum();
                    let rms = (sum_squares / data.len() as f32).sqrt();
                    let alpha = 0.2;
                    if let Ok(mut lvl) = audio_clone.lock() {
                        *lvl = alpha * rms + (1.0 - alpha) * *lvl;
                    }
                },
                err_fn,
                Some(Duration::from_millis(100)),
            )
            .expect("Failed to build input stream");
        stream.play().expect("Failed to start stream");

        // Keep thread alive
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    return audio_level;
}
