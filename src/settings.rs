// -----------------------------------------------------------------------------
// settings.rs — minimal settings with announce_weather + JSON preserved
// -----------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub location: String,

    // JSON key is "weather_announce" — we map it to announce_weather internally
    #[serde(rename = "weather_announce")]
    pub announce_weather: bool,

    pub measurement: String,
    pub weather_refresh_minutes: i32,

    pub clock_size: i32,
    pub clock_size_choices: Vec<i32>,

    // SESSION-ONLY
    #[serde(skip)]
    pub diagnostics: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            location: "01532".into(),
            announce_weather: false,
            measurement: "fahrenheit".into(),
            weather_refresh_minutes: 15,
            clock_size: 200,
            clock_size_choices: vec![200, 250, 300, 400],
            diagnostics: false,
        }
    }
}

pub fn load_settings() -> Settings {
    if let Ok(data) = fs::read_to_string("settings.json") {
        if let Ok(mut s) = serde_json::from_str::<Settings>(&data) {
            s.diagnostics = false;
            return s;
        }
    }
    Settings::default()
}

pub fn save_settings(settings: &Settings) {
    let mut clone = settings.clone();
    clone.diagnostics = false;

    if let Ok(json) = serde_json::to_string_pretty(&clone) {
        let _ = fs::write("settings.json", json);
    }
}
