#![allow(dead_code)]

use clap::ValueEnum;

#[derive(Clone, ValueEnum, PartialEq)]
pub enum InputUnit {
    #[value(name = "m")]
    M,
    #[value(name = "ft")]
    Ft,
}

#[derive(Clone)]
pub struct OutputUnit {
    pub km: f64, // Kilometers
    pub mi: f64, // Miles
    pub nm: f64, // Nautical Miles
    pub m: f64,  // Meters
    pub ft: f64, // Feet
}

pub fn unit_converter(result: f64) -> OutputUnit {
    OutputUnit {
        km: result / 1000.0,
        mi: result / 1609.344,
        nm: result / 1852.0,
        m: result,
        ft: result / 0.3048,
    }
}

pub fn input_ft_to_meters(x: f64) -> f64 {
    x * 0.3048
}
