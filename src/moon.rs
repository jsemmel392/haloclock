// -----------------------------------------------------------------------------
// moon.rs — Elegant sun + elegant moon phases
// Pure crescent geometry (no disks unless intentional)
// Parameter-driven, clean, minimal, readable
// New Moon uses a thick adjustable ring ("O" style)
// Full USNO astronomy API (phase, illum, rise/set) + compat shim
// -----------------------------------------------------------------------------

use eframe::egui::{Color32, Pos2, Painter, Stroke};

// -----------------------------------------------------------------------------
// Sun parameters + renderer
// -----------------------------------------------------------------------------

pub struct SunParams {
    pub ray_length: f32,
    pub ray_thickness: f32,
    pub ray_count: usize,
}

pub const ELEGANT_SUN: SunParams = SunParams {
    ray_length: 0.35,
    ray_thickness: 2.0,
    ray_count: 8,
};

pub fn draw_sun(
    p: &Painter,
    pos: Pos2,
    size: f32,
    color: Color32,
    params: &SunParams,
) {
    let r = size / 2.0;

    // Sun disk
    p.circle_filled(pos, r * 0.65, color);

    // Rays
    let ray_len = r * params.ray_length;
    let ray_thick = params.ray_thickness;

    for i in 0..params.ray_count {
        let angle = (i as f32) * (std::f32::consts::TAU / params.ray_count as f32);
        let dx = angle.cos();
        let dy = angle.sin();

        let start = Pos2::new(pos.x + dx * (r * 0.75), pos.y + dy * (r * 0.75));
        let end   = Pos2::new(pos.x + dx * (r * 0.75 + ray_len), pos.y + dy * (r * 0.75 + ray_len));

        p.line_segment([start, end], Stroke::new(ray_thick, color));
    }
}

// -----------------------------------------------------------------------------
// Moon phase parameters + renderer
// -----------------------------------------------------------------------------

pub struct PhaseParams {
    pub yellow_offset: f32,     // shift of yellow circle
    pub carve_offset: f32,      // shift of carve circle
    pub carve_scale: f32,       // size of carve circle

    // Only used for New Moon
    pub newmoon_thickness: f32, // fraction of radius (0.10–0.40 recommended)
}

// Pure crescent renderer (no disk)
pub fn draw_phase(
    p: &Painter,
    pos: Pos2,
    size: f32,
    color: Color32,
    bg: Color32,
    params: &PhaseParams,
) {
    let r = size / 2.0;

    // -------------------------------------------------------------------------
    // NEW MOON: thick outlined circle ("O" style)
    // -------------------------------------------------------------------------
    if params.carve_scale == 1.0 && params.yellow_offset == 0.0 {
        p.circle_stroke(
            pos,
            r,
            Stroke::new(r * params.newmoon_thickness, color),
        );
        return;
    }

    // -------------------------------------------------------------------------
    // Normal crescent logic
    // -------------------------------------------------------------------------

    // Yellow circle (source of crescent)
    let yellow_center = Pos2::new(pos.x + params.yellow_offset * r, pos.y);
    p.circle_filled(yellow_center, r, color);

    // Carve shadow
    let carve_center = Pos2::new(pos.x + params.carve_offset * r, pos.y);
    p.circle_filled(carve_center, r * params.carve_scale, bg);
}

// -----------------------------------------------------------------------------
// All 8 elegant moon phases
// -----------------------------------------------------------------------------

pub const PHASES: [PhaseParams; 8] = [
    // 0 — New Moon (O-ring)
    PhaseParams {
        yellow_offset: 0.0,
        carve_offset: 0.0,
        carve_scale: 1.0,
        newmoon_thickness: 0.03, // tweak this to taste
    },

    // 1 — Waxing Crescent
    PhaseParams {
        yellow_offset: -0.25,
        carve_offset: 0.40,
        carve_scale: 0.95,
        newmoon_thickness: 0.0,
    },

    // 2 — First Quarter
    PhaseParams {
        yellow_offset: -0.50,
        carve_offset: 0.00,
        carve_scale: 0.95,
        newmoon_thickness: 0.0,
    },

    // 3 — Waxing Gibbous
    PhaseParams {
        yellow_offset: -0.60,
        carve_offset: -0.30,
        carve_scale: 0.95,
        newmoon_thickness: 0.0,
    },

    // 4 — Full Moon (no carve)
    PhaseParams {
        yellow_offset: 0.0,
        carve_offset: 3.0,
        carve_scale: 0.01,
        newmoon_thickness: 0.0,
    },

    // 5 — Waning Gibbous
    PhaseParams {
        yellow_offset: 0.60,
        carve_offset: 0.30,
        carve_scale: 0.95,
        newmoon_thickness: 0.0,
    },

    // 6 — Last Quarter
    PhaseParams {
        yellow_offset: 0.50,
        carve_offset: 0.00,
        carve_scale: 0.95,
        newmoon_thickness: 0.0,
    },

    // 7 — Waning Crescent
    PhaseParams {
        yellow_offset: 0.25,
        carve_offset: -0.40,
        carve_scale: 0.95,
        newmoon_thickness: 0.0,
    },
];

// -----------------------------------------------------------------------------
// USNO Moon (full astronomy API, synced with truth_checker.py)
// -----------------------------------------------------------------------------

use chrono::Local;
use serde::Deserialize;

// This matches truth_checker.py:
// data["properties"]["data"]["curphase"]
// data["properties"]["data"]["fracillum"]
// data["properties"]["data"]["moondata"]
// data["properties"]["data"]["sundata"]

#[derive(Deserialize)]
struct UsnoRoot {
    properties: UsnoProperties,
}

#[derive(Deserialize)]
struct UsnoProperties {
    data: UsnoData,
}

#[derive(Deserialize)]
struct UsnoData {
    curphase: String,
    fracillum: String,
    moondata: Vec<UsnoPhenTime>,
    sundata: Vec<UsnoPhenTime>,
}

#[derive(Deserialize)]
struct UsnoPhenTime {
    phen: String,
    time: String,
}

// Public struct you can store in ClockApp later
pub struct UsnoMoon {
    pub index: usize,
    pub phase: String,
    pub illum: String,
    pub moon_rise: Option<String>,
    pub moon_set: Option<String>,
    pub sun_rise: Option<String>,
    pub sun_set: Option<String>,
}

// Same mapping as before
fn map_usno_phase_to_index(s: &str) -> usize {
    match s {
        "New Moon"        => 0,
        "Waxing Crescent" => 1,
        "First Quarter"   => 2,
        "Waxing Gibbous"  => 3,
        "Full Moon"       => 4,
        "Waning Gibbous"  => 5,
        "Last Quarter"    => 6,
        "Waning Crescent" => 7,
        _                 => 0, // fallback to New Moon
    }
}

/// Full USNO fetch, synced with truth_checker.py
pub async fn fetch_usno_moon(
    lat: f64,
    lon: f64,
) -> Result<UsnoMoon, Box<dyn std::error::Error>> {
    let today = Local::now().format("%Y-%m-%d");

    let url = format!(
        "https://aa.usno.navy.mil/api/rstt/oneday?date={}&coords={},{}",
        today, lat, lon
    );

    let root = reqwest::get(url).await?.json::<UsnoRoot>().await?;
    let props = root.properties.data;

    let mut moon_rise: Option<String> = None;
    let mut moon_set: Option<String> = None;
    for item in props.moondata {
        match item.phen.as_str() {
            "Rise" => moon_rise = Some(item.time),
            "Set"  => moon_set  = Some(item.time),
            _      => {}
        }
    }

    let mut sun_rise: Option<String> = None;
    let mut sun_set: Option<String> = None;
    for item in props.sundata {
        match item.phen.as_str() {
            "Rise" => sun_rise = Some(item.time),
            "Set"  => sun_set  = Some(item.time),
            _      => {}
        }
    }

    let phase = props.curphase;
    let illum = props.fracillum;
    let index = map_usno_phase_to_index(&phase);

    Ok(UsnoMoon {
        index,
        phase,
        illum,
        moon_rise,
        moon_set,
        sun_rise,
        sun_set,
    })
}

/// Compat shim for existing clock.rs:
/// still returns (index, illum) so you don't break anything yet.
pub async fn fetch_usno_phase(
    lat: f64,
    lon: f64,
) -> Result<(usize, String), Box<dyn std::error::Error>> {
    let m = fetch_usno_moon(lat, lon).await?;
    Ok((m.index, m.illum.clone()))
}
