mod audio;

use audio::get_audio_level;
use std::thread;
use std::time::Duration;

// use eframe::{egui, App, CreationContext, NativeOptions};

// struct MyApp {
//     label: String,
//     audio_level: Arc<Mutex<f32>>,
// }

// impl Default for MyApp {
//     fn default() -> Self {
//         Self {
//             label: "Hello, macOS!".to_owned(),
//             audio_level: Arc::new(Mutex::new(0.0)),
//         }
//     }
// }

// impl App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Simple Rust UI with Audio Capture");
//             ui.label(&self.label);

//             // Display audio level
//             if let Ok(level) = self.audio_level.lock() {
//                 ui.label(format!("Audio level (RMS): {:.3}", *level));
//             }

//             if ui.button("Click me").clicked() {
//                 self.label = "Button clicked!".to_owned();
//             }
//         });
//         ctx.request_repaint(); // continuously update UI
//     }
// }

fn main() {
    let audio_level = get_audio_level();
    let interval = Duration::from_millis(250);

    loop {
        if let Ok(level) = audio_level.lock() {
            if *level > 0.2 {
                println!("Current audio RMS: {}", *level);
            }
        }
        thread::sleep(interval);
    }
    // Launch the UI app, passing in the audio level Arc
    // let app = MyApp {
    //     label: "Listening to audio...".to_owned(),
    //     audio_level,
    // };

    // let native_options = NativeOptions::default();
    // eframe::run_native(
    //     "Simple eframe Audio App",
    //     native_options,
    //     Box::new(|_cc: &CreationContext| Box::new(app)),
    // );
}

/*
Instructions:
1. Install Rust and Cargo via https://rustup.rs/
2. Install a system audio loopback device (e.g., BlackHole) and set it as the default input.
3. Create the project and replace files:
   ```bash
   cargo new simple_ui_app
   cd simple_ui_app
   ```
4. Replace Cargo.toml and src/main.rs with the content above.
5. Build and run on macOS:
   ```bash
   cargo run --release
   ```
This app captures system audio input and displays the real-time RMS level in the UI.
*/
