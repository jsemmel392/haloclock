// -----------------------------------------------------------------------------
// config.rs — clock_face v1.2
// Centralized configuration for all tweakable visual parameters.
// Every value is adjustable. No magic numbers in main.rs.
// -----------------------------------------------------------------------------

use eframe::egui::Color32;

// Version
pub const VERSION: &str = "1.2";

// New variables
pub const CLOCK_BG_COLOR: Color32 = Color32::from_rgb(20, 20, 20);
pub static CLOCK_BORDER_COLOR: Color32 = Color32::from_gray(200);


// pub const CLOCK_BORDER_THICKNESS: f32 = 12.0; //400

pub const CLOCK_BORDER_THICKNESS: f32 = 9.0; //200



// -----------------------------
// Clock Geometry
// -----------------------------
pub const CLOCK_RADIUS_FACTOR: f32 = 0.45;

// Tick marks
pub const TICK_INNER_FACTOR: f32 = 0.88;
pub const TICK_INNER_MAJOR_FACTOR: f32 = 0.78;
pub const TICK_OUTER_FACTOR: f32 = 0.95;

pub const TICK_THICKNESS: f32 = 1.5;
pub const TICK_THICKNESS_MAJOR: f32 = 3.0;

pub const TICK_COLOR: Color32 = Color32::from_gray(180); // WHITE

// -----------------------------
// Numbers (1–12)
// -----------------------------
//pub const NUMBER_FONT_SIZE: f32 = 32.0; //400
pub const NUMBER_FONT_SIZE: f32 = 20.0; //200

pub const NUMBER_COLOR: Color32 = Color32::from_gray(200); // WHITE
pub const NUMBER_DISTANCE_FACTOR: f32 = 0.65;  // was 0.70

// -----------------------------
// Hands
// -----------------------------

pub const HOUR_HAND_LENGTH: f32 = 0.50;
pub const HOUR_HAND_WIDTH: f32 = 7.0;  //5.0

pub const MINUTE_HAND_LENGTH: f32 = 0.70;
pub const MINUTE_HAND_WIDTH: f32 = 5.0; //3.0

pub const SECOND_HAND_LENGTH: f32 = 0.80;
pub const SECOND_HAND_WIDTH: f32 = 3.0;  //1.5 , 2.0

pub const HAND_TAIL_FACTOR: f32 = -0.10;

pub const HOUR_HAND_COLOR: Color32 = Color32::WHITE;
pub const MINUTE_HAND_COLOR: Color32 = Color32::WHITE;
pub const SECOND_HAND_COLOR: Color32 = Color32::RED;

// -----------------------------
// Digital Clock Insert
// -----------------------------
// pub const DIGITAL_FONT_SIZE: f32 = 20.0; //400
pub const DIGITAL_FONT_SIZE: f32 = 12.0;


// pub const DIGITAL_TEXT_COLOR: Color32 = Color32::WHITE;
pub const DIGITAL_TEXT_COLOR: Color32 = Color32::from_rgb(255, 180, 100);


pub const DIGITAL_BG_COLOR: Color32 = Color32::from_gray(30);

//pub const DIGITAL_OFFSET_FACTOR: f32 = 0.35; //400
pub const DIGITAL_OFFSET_FACTOR: f32 = 0.30; //200

pub const DIGITAL_WIDTH_FACTOR: f32 = 0.90;
pub const DIGITAL_HEIGHT: f32 = 32.0;
pub const DIGITAL_CORNER_RADIUS: f32 = 6.0;
