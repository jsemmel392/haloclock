#![windows_subsystem = "windows"]
// -----------------------------------------------------------------------------
//
// Entry point. Owns settings.json lifecycle:
// - loads settings.json (or defaults)
// - ensures it's ready
// - passes Settings into clock::run_with_settings()
// -----------------------------------------------------------------------------

mod config;
mod settings;
mod moon;
//mod ui_settings;
mod weather;
mod clock;
mod talking;
mod player; 
mod sounds;

// Testing GitHub Actions


use settings::{load_settings, save_settings, Settings};

fn main() {
    // Load settings (or defaults)
    //let mut settings: Settings = load_settings();
	let settings: Settings = load_settings();


    // Persist defaults if we just fell back
    // (simple heuristic: if file didn't exist, user will get defaults written)
    // You can refine this later if you want explicit file_exists tracking.
    save_settings(&settings);

    clock::run_with_settings(settings);
}
