// SPDX-License-Identifier: MIT OR Apache-2.0

//! detect-range
//! Author: Rich Davs
//!
//! Radar Detection line-of-sight calculator utilising the
//! effective Earth radius / k-factor model commonly used in Radar Engineering.

use clap::Parser;
use owo_colors::OwoColorize;
use std::io;
mod helpers;
use crate::helpers::*;

const EARTH_RADIUS: f64 = 6371008.7714;
const K_FACTOR: f64 = 4.0 / 3.0;
const FEET_TO_METERS: f64 = 0.3048;

#[derive(Parser)]
#[command(
    name = "detect-range",
     version,
     author,
     about,
    after_help = "Operation:\n detect-range     Run in interactive mode\n detect-range --hr <height of radar> --hru <unit m/ft> --ht <height of target> --htu <unit m/ft>      Run in CLI mode")]
struct Cli {
    #[arg(
        long,
        value_name = "HEIGHT_RADAR",
        group = "cli_mode",
        requires = "hru"
    )]
    /// Height of Radar in Meters (AGL)
    hr: Option<f64>,

    #[arg(long, value_name = "HEIGHT_RADAR_UNIT", group = "cli_mode")]
    /// Radar Input Unit (AGL)
    hru: Option<InputUnit>,

    #[arg(
        long,
        short = 't',
        value_name = "HEIGHT_TARGET",
        group = "cli_mode",
        requires = "htu"
    )]
    /// Height of Target in feet (AGL)
    ht: Option<f64>,

    #[arg(long, value_name = "HEIGHT_TARGET_UNIT", group = "cli_mode")]
    /// Target Input Unit
    htu: Option<InputUnit>,
}

fn main() {
    let args = Cli::parse();

    match (args.hr, args.hru, args.ht, args.htu) {
        (None, None, None, None) => {
            interactive_mode_flow();
        }

        (Some(hr), Some(hru), Some(ht), Some(htu)) => {
            let hrm = if hru == InputUnit::Ft {
                input_ft_to_meters(hr)
            } else {
                hr
            };
            let htm = if htu == InputUnit::Ft {
                input_ft_to_meters(ht)
            } else {
                ht
            };
            let result_meters: f64 = radar_horizon(hrm, htm);
            let output: OutputUnit = unit_converter(result_meters);
            println!("The target will be detected at: ");
            println!("{} NM", format!("{:.2}", output.nm).green());
            println!("{} KM", format!("{:.2}", output.km).green());
        }
        _ => {
            std::process::exit(2);
        }
    }
}

fn interactive_mode_flow() {
    welcome();
    let hrm: f64 = user_input_radar();
    let htm: f64 = user_input_tgt();
    let result_meters: f64 = radar_horizon(hrm, htm);
    let output: OutputUnit = unit_converter(result_meters);
    println!("The target will be detected at: ");
    println!("{} NM", format!("{:.2}", output.nm).green());
    println!("{} KM", format!("{:.2}", output.km).green());
}

fn welcome() {
    println!("\n");
    println!("Welcome to the Radar Horizon Tool\n");
    println!("Enter Radar Antenna Height and Target Aircraft Height to Assess Detection Range\n")
}

fn user_input_radar() -> f64 {
    loop {
        println!("Input Radar Antenna Height (AGL) in meters: ");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        match input.trim().parse::<f64>() {
            Ok(f64) => return f64,
            Err(_) => println!("Invalid please enter a number only e.g 250"),
        }
    }
}

fn user_input_tgt() -> f64 {
    loop {
        println!("Input Aircraft Height in ft (AGL):");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        match input.trim().parse::<f64>() {
            Ok(f64) => return f64 * FEET_TO_METERS,
            Err(_) => println!("Invalid please enter a whole number e.g 250"),
        }
    }
}

fn radar_horizon(hrm: f64, htm: f64) -> f64 {
    let hr_calc: f64 = 2.0 * (EARTH_RADIUS * K_FACTOR) * hrm;
    let ht_calc: f64 = 2.0 * (EARTH_RADIUS * K_FACTOR) * htm;

    let root_hr: f64 = (hr_calc).sqrt();
    let root_ht: f64 = (ht_calc).sqrt();

    root_hr + root_ht
}
