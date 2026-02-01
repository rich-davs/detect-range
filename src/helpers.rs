#![allow(dead_code)]

use clap::ValueEnum;
use serde::Serialize;

pub const EARTH_RADIUS: f64 = 6371008.7714;
pub const K_FACTOR: f64 = 4.0 / 3.0;
pub const FEET_TO_METERS: f64 = 0.3048;
pub const METERS_TO_KM: f64 = 1000.0; // Usage - {INPUT METERS} / METERS_TO_KM = KM
pub const METERS_TO_NM: f64 = 1852.0; // Usage - {INPUT METERS} / METERS_TO_NM = NM
pub const METERS_TO_FT: f64 = 0.3048; // Usage - {INPUT METERS} / METERS_TO_FT = FT
pub const METERS_TO_MI: f64 = 1609.344; // Usage - {INPUT METERS} / METERS_TO_MI = MI

#[derive(Clone, ValueEnum, PartialEq, Serialize, Copy)]
pub enum InputUnit {
    #[serde(rename = "m")]
    #[value(name = "m")]
    M,
    #[serde(rename = "ft")]
    #[value(name = "ft")]
    Ft,
}

#[derive(Clone, Serialize)]
pub struct OutputUnit {
    pub km: f64, // Kilometers
    pub mi: f64, // Miles
    pub nm: f64, // Nautical Miles
    pub m: f64,  // Meters
    pub ft: f64, // Feet
}

pub fn unit_converter(result: f64) -> OutputUnit {
    OutputUnit {
        km: result / METERS_TO_KM,
        mi: result / METERS_TO_MI,
        nm: result / METERS_TO_NM,
        m: result,
        ft: result / METERS_TO_FT,
    }
}

pub fn resolve_input_m(value: f64, unit: InputUnit) -> f64 {
    match unit {
        InputUnit::M => value,
        InputUnit::Ft => value * FEET_TO_METERS,
    }
}
