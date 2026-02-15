// -----------------------------------------------------------------------------
// sounds.rs — talking_clock version 1.2
// Embeds all WAV fragments and provides clean playback functions.
// -----------------------------------------------------------------------------

use crate::player::play_wav_bytes;

// -----------------------------------------------------------------------------
// GREETING WAVs
// -----------------------------------------------------------------------------
pub const GOOD_MORNING: &[u8]   = include_bytes!("../Sounds/good_morning.wav");
pub const GOOD_AFTERNOON: &[u8] = include_bytes!("../Sounds/good_afternoon.wav");
pub const GOOD_EVENING: &[u8]   = include_bytes!("../Sounds/good_evening.wav");
pub const GOOD_NIGHT: &[u8]     = include_bytes!("../Sounds/good_night.wav");

// -----------------------------------------------------------------------------
// TIME PHRASE WAVs
// -----------------------------------------------------------------------------
pub const TIME_IS: &[u8] = include_bytes!("../Sounds/time_is.wav");
pub const AM: &[u8]      = include_bytes!("../Sounds/AM.wav");
pub const PM: &[u8]      = include_bytes!("../Sounds/PM.wav");
pub const OH: &[u8]      = include_bytes!("../Sounds/oh.wav");

// -----------------------------------------------------------------------------
// TEMPERATURE PHRASE WAVs
// -----------------------------------------------------------------------------
pub const TEMP_IS: &[u8]    = include_bytes!("../Sounds/temperature_is.wav");
pub const FEELS_LIKE: &[u8] = include_bytes!("../Sounds/feels_like.wav");
pub const DEGREES: &[u8]    = include_bytes!("../Sounds/degrees.wav");

// -----------------------------------------------------------------------------
// NUMBER WAVs (0–20, tens, 100)
// -----------------------------------------------------------------------------
pub const N0: &[u8]   = include_bytes!("../Sounds/0.wav");
pub const N1: &[u8]   = include_bytes!("../Sounds/1.wav");
pub const N2: &[u8]   = include_bytes!("../Sounds/2.wav");
pub const N3: &[u8]   = include_bytes!("../Sounds/3.wav");
pub const N4: &[u8]   = include_bytes!("../Sounds/4.wav");
pub const N5: &[u8]   = include_bytes!("../Sounds/5.wav");
pub const N6: &[u8]   = include_bytes!("../Sounds/6.wav");
pub const N7: &[u8]   = include_bytes!("../Sounds/7.wav");
pub const N8: &[u8]   = include_bytes!("../Sounds/8.wav");
pub const N9: &[u8]   = include_bytes!("../Sounds/9.wav");
pub const N10: &[u8]  = include_bytes!("../Sounds/10.wav");
pub const N11: &[u8]  = include_bytes!("../Sounds/11.wav");
pub const N12: &[u8]  = include_bytes!("../Sounds/12.wav");
pub const N13: &[u8]  = include_bytes!("../Sounds/13.wav");
pub const N14: &[u8]  = include_bytes!("../Sounds/14.wav");
pub const N15: &[u8]  = include_bytes!("../Sounds/15.wav");
pub const N16: &[u8]  = include_bytes!("../Sounds/16.wav");
pub const N17: &[u8]  = include_bytes!("../Sounds/17.wav");
pub const N18: &[u8]  = include_bytes!("../Sounds/18.wav");
pub const N19: &[u8]  = include_bytes!("../Sounds/19.wav");
pub const N20: &[u8]  = include_bytes!("../Sounds/20.wav");

pub const N30: &[u8]  = include_bytes!("../Sounds/30.wav");
pub const N40: &[u8]  = include_bytes!("../Sounds/40.wav");
pub const N50: &[u8]  = include_bytes!("../Sounds/50.wav");
pub const N60: &[u8]  = include_bytes!("../Sounds/60.wav");
pub const N70: &[u8]  = include_bytes!("../Sounds/70.wav");
pub const N80: &[u8]  = include_bytes!("../Sounds/80.wav");
pub const N90: &[u8]  = include_bytes!("../Sounds/90.wav");

pub const N100: &[u8] = include_bytes!("../Sounds/100.wav");

// -----------------------------------------------------------------------------
// MISC WAVs
// -----------------------------------------------------------------------------
pub const JOSEPH: &[u8]   = include_bytes!("../Sounds/Joseph.wav");
pub const POSITIVE: &[u8] = include_bytes!("../Sounds/positive.wav");
pub const NEGATIVE: &[u8] = include_bytes!("../Sounds/negative.wav");

// -----------------------------------------------------------------------------
// PLAY NUMBER (0–100) WITH COMPOSITE SUPPORT
// -----------------------------------------------------------------------------
pub fn play_number(n: i32) {
    if n < 0 || n > 100 {
        return;
    }

    match n {
        0  => play_wav_bytes(N0),
        1  => play_wav_bytes(N1),
        2  => play_wav_bytes(N2),
        3  => play_wav_bytes(N3),
        4  => play_wav_bytes(N4),
        5  => play_wav_bytes(N5),
        6  => play_wav_bytes(N6),
        7  => play_wav_bytes(N7),
        8  => play_wav_bytes(N8),
        9  => play_wav_bytes(N9),
        10 => play_wav_bytes(N10),
        11 => play_wav_bytes(N11),
        12 => play_wav_bytes(N12),
        13 => play_wav_bytes(N13),
        14 => play_wav_bytes(N14),
        15 => play_wav_bytes(N15),
        16 => play_wav_bytes(N16),
        17 => play_wav_bytes(N17),
        18 => play_wav_bytes(N18),
        19 => play_wav_bytes(N19),
        20 => play_wav_bytes(N20),
        30 => play_wav_bytes(N30),
        40 => play_wav_bytes(N40),
        50 => play_wav_bytes(N50),
        60 => play_wav_bytes(N60),
        70 => play_wav_bytes(N70),
        80 => play_wav_bytes(N80),
        90 => play_wav_bytes(N90),
        100 => play_wav_bytes(N100),

        _ => {
            let tens = (n / 10) * 10;
            let ones = n % 10;

            play_number(tens);

            if ones > 0 {
                play_number(ones);
            }
        }
    }
}

// -----------------------------------------------------------------------------
// GREETING
// -----------------------------------------------------------------------------
pub fn play_greeting(period: &str) {
    match period {
        "morning"   => play_wav_bytes(GOOD_MORNING),
        "afternoon" => play_wav_bytes(GOOD_AFTERNOON),
        "evening"   => play_wav_bytes(GOOD_EVENING),
        _           => play_wav_bytes(GOOD_NIGHT),
    }
}

// -----------------------------------------------------------------------------
// TIME (12 oh 7 format)
// -----------------------------------------------------------------------------
pub fn play_time(hour: u32, minute: u32, is_pm: bool) {
    play_wav_bytes(TIME_IS);
    play_number(hour as i32);

    if minute < 10 {
        play_wav_bytes(OH);
        play_number(minute as i32);
    } else {
        play_number(minute as i32);
    }

    if is_pm {
        play_wav_bytes(PM);
    } else {
        play_wav_bytes(AM);
    }
}

// -----------------------------------------------------------------------------
// TEMPERATURE (supports negative values)
// -----------------------------------------------------------------------------
pub fn play_temperature(temp: i32) {
    play_wav_bytes(TEMP_IS);

    if temp < 0 {
        play_wav_bytes(NEGATIVE);
        play_number(temp.abs());
    } else {
        play_number(temp);
    }

    play_wav_bytes(DEGREES);
}

// -----------------------------------------------------------------------------
// FEELS LIKE (supports negative values)
// -----------------------------------------------------------------------------
pub fn play_feels_like(feels: i32) {
    play_wav_bytes(FEELS_LIKE);

    if feels < 0 {
        play_wav_bytes(NEGATIVE);
        play_number(feels.abs());
    } else {
        play_number(feels);
    }

    play_wav_bytes(DEGREES);
}
