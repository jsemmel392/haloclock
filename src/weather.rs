// File: src/weather.rs
// Description: ZIP lookup + Open-Meteo weather with retries and 30s timeout.
// Returns city name from ZIP. All constants at top. No drift.

use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::{Duration, Instant};
use std::{thread};

// ---------------------------
// CONSTANTS
// ---------------------------

const ZIP_URL: &str = "https://api.zippopotam.us/us/";
const METEO_URL: &str = "https://api.open-meteo.com/v1/forecast";

const RETRY_ATTEMPTS: usize = 5;
const RETRY_DELAYS_MS: [u64; RETRY_ATTEMPTS] = [0, 200, 400, 800, 1200];

const MAX_TOTAL_TIME_MS: u128 = 30_000; // 30 seconds

// ---------------------------
// ZIP LOOKUP STRUCTS
// ---------------------------

#[derive(Deserialize)]
struct ZipPlace {
    #[serde(rename = "place name")]
    place_name: String,

    latitude: String,
    longitude: String,
}

#[derive(Deserialize)]
struct ZipResponse {
    places: Vec<ZipPlace>,
}

// ---------------------------
// WEATHER STRUCTS
// ---------------------------

#[derive(Deserialize)]
struct Current {
    temperature_2m: f32,
    apparent_temperature: f32,
    relative_humidity_2m: Option<f32>,
    is_day: u8,
}

#[derive(Deserialize)]
struct Daily {
    temperature_2m_max: Vec<f32>,
    temperature_2m_min: Vec<f32>,
}

#[derive(Deserialize)]
struct WeatherResponse {
    current: Current,
    daily: Daily,
}

pub struct Weather {
    pub temperature: f32,
    pub feels_like: f32,
    pub humidity: Option<f32>,
    pub high: f32,
    pub low: f32,
    pub is_day: bool,
    pub location_name: String,   // NEW
	pub lat: f32, 
	pub lon: f32,
}

// ---------------------------
// HELPERS
// ---------------------------

fn c_to_f(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}

// ---------------------------
// ZIP → COORDS + CITY
// ---------------------------

fn zip_to_coords(zip: &str, client: &Client) -> Result<(f32, f32, String), String> {
    let url = format!("{}{}", ZIP_URL, zip);

    let resp = client
        .get(&url)
        .send()
        .map_err(|e| format!("ZIP lookup HTTP error: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("ZIP lookup failed: HTTP {}", resp.status()));
    }

    let data: ZipResponse = resp
        .json()
        .map_err(|e| format!("ZIP JSON parse error: {}", e))?;

    let place = data.places.get(0).ok_or("ZIP lookup returned no places")?;

    let lat: f32 = place.latitude.parse().map_err(|_| "Invalid latitude")?;
    let lon: f32 = place.longitude.parse().map_err(|_| "Invalid longitude")?;
    let city = place.place_name.clone();

    Ok((lat, lon, city))
}

// ---------------------------
// RETRY WRAPPER
// ---------------------------

fn fetch_with_retry(url: &str, client: &Client) -> Result<String, String> {
    for (attempt, delay) in RETRY_DELAYS_MS.iter().enumerate() {
        if *delay > 0 {
            thread::sleep(Duration::from_millis(*delay));
        }

        let resp = client.get(url).send();

        match resp {
            Ok(r) => {
                if r.status().is_success() {
                    return r.text().map_err(|e| format!("Read error: {}", e));
                } else {
                    println!(
                        "Open-Meteo attempt {} failed: HTTP {}",
                        attempt + 1,
                        r.status()
                    );
                }
            }
            Err(e) => {
                println!("Open-Meteo attempt {} error: {}", attempt + 1, e);
            }
        }
    }

    Err("Open-Meteo failed after 5 attempts".into())
}

// ---------------------------
// MAIN WEATHER FUNCTION
// ---------------------------

pub fn get_weather(zip: &str, measurement: &str) -> Result<Weather, String> {
    let start = Instant::now();
    let client = Client::new();

    // ZIP → coords + city
    let (lat, lon, city) = zip_to_coords(zip, &client)?;

    if start.elapsed().as_millis() > MAX_TOTAL_TIME_MS {
        return Err("Weather lookup exceeded 30 seconds (ZIP stage)".into());
    }

    // Build Open-Meteo URL (auto timezone)
    let url = format!(
        "{}?latitude={}&longitude={}&current=temperature_2m,apparent_temperature,is_day,relative_humidity_2m&daily=temperature_2m_max,temperature_2m_min&timezone=auto",
        METEO_URL, lat, lon
    );

    // Fetch with retry
    let body = fetch_with_retry(&url, &client)?;

    if start.elapsed().as_millis() > MAX_TOTAL_TIME_MS {
        return Err("Weather lookup exceeded 30 seconds (Open-Meteo stage)".into());
    }

    // Parse JSON
    let data: WeatherResponse =
        serde_json::from_str(&body).map_err(|e| format!("JSON parse error: {}", e))?;

    // Convert if needed
    let (temp, feels, high, low) = if measurement == "fahrenheit" {
        (
            c_to_f(data.current.temperature_2m),
            c_to_f(data.current.apparent_temperature),
            c_to_f(data.daily.temperature_2m_max[0]),
            c_to_f(data.daily.temperature_2m_min[0]),
        )
    } else {
        (
            data.current.temperature_2m,
            data.current.apparent_temperature,
            data.daily.temperature_2m_max[0],
            data.daily.temperature_2m_min[0],
        )
    };

    Ok(Weather {
        temperature: temp,
        feels_like: feels,
        humidity: data.current.relative_humidity_2m,
        high,
        low,
        is_day: data.current.is_day == 1,
        location_name: city,   // NEW
		lat,
		lon,
    })
}
