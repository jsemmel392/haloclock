// -----------------------------------------------------------------------------
// talking.rs — UI Clock Speech Integration (v1.40)
// -----------------------------------------------------------------------------

use chrono::{Local, Timelike};
use crate::weather;
use crate::sounds::*;
use crate::player::*;
use crate::settings::Settings;

pub fn speak_now(settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    // TIME
    let announce_weather = false;     // Temporary overide to stop speaking weather when false
	let now = Local::now();
    let hour24 = now.hour();
    let minute = now.minute();

    let hour12 = match hour24 {
        0 => 12,
        13..=23 => hour24 - 12,
        _ => hour24,
    };

    let is_pm = hour24 >= 12;

    let greeting = match hour24 {
        5..=11 => "morning",
        12..=16 => "afternoon",
        17..=20 => "evening",
        _ => "night",
    };

    // WEATHER (UI clock uses announce_weather)
    let mut temp_opt = None;
    let mut feels_opt = None;

    if settings.announce_weather  && announce_weather{
        let w = weather::get_weather(&settings.location, &settings.measurement)?;
        temp_opt = Some(w.temperature.round() as i32);
        feels_opt = Some(w.feels_like.round() as i32);
		println!("announce_weather = {}", settings.announce_weather);

    }

    // SPEAK
    play_greeting(greeting);
    play_time(hour12, minute, is_pm);

    if let Some(t) = temp_opt {
        play_temperature(t);
    }
    if let Some(f) = feels_opt {
        play_feels_like(f);
    }

    Ok(())
}
