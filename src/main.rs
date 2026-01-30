// SPDX-License-Identifier: MIT OR Apache-2.0

//! detect-range
//! Author: Rich Davs
//!
//! Radar Detection line-of-sight calculator utilising the
//! effective Earth radius / k-factor model commonly used in Radar Engineering.

use clap::Parser;
use owo_colors::OwoColorize;
use std::io;

const EARTH_RADIUS: f64 = 6371008.7714;
const K_FACTOR: f64 = 4.0 / 3.0;
const NM_CONVERSION_FACTOR: f64 = 1852.0;
const FEET_TO_METERS: f64 = 0.3048;

#[derive(Parser)]
#[command(
    name = "detect-range",
     version,
     author,
     about,
     group(
        clap::ArgGroup::new("cli_mode")
        .required(false)
        .multiple(true)
        .requires_all(["hr", "ht"])
     ),
    after_help = "Operation:\n detect-range     Run in interactive mode\n detect-range --hr <meters> --ht <ft>      Run in CLI mode")]
struct Cli {
    #[arg(
        long,
        short = 'r',
        value_name = "HEIGHT_RADAR_METERS",
        group = "cli_mode"
    )]
    /// Height of Radar in Meters (AGL)
    hr: Option<f64>,

    #[arg(
        long,
        short = 't',
        value_name = "HEIGHT_TARGET_FEET",
        group = "cli_mode"
    )]
    /// Height of Target in feet (AGL)
    ht: Option<f64>,
}

fn main() {
    let args = Cli::parse();

    if args.hr.is_none() && args.ht.is_none() {
        welcome();
        let hr: f64 = user_input_radar();
        let ht: f64 = user_input_tgt();
        let range: u64 = radar_horizon(hr, ht) as u64;
        println!(
            "Target will be detected at {range} NM from Radar",
            range = range.to_string().green()
        );
    } else {
        let hr: f64 = args.hr.unwrap();
        let ht: f64 = args.ht.unwrap() * FEET_TO_METERS;
        let range: u64 = radar_horizon(hr, ht) as u64;
        println!(
            "Target will be detected at {range} NM from Radar",
            range = range.to_string().green()
        );
    }
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

fn radar_horizon(hr: f64, ht: f64) -> f64 {
    let hr_calc: f64 = 2.0 * (EARTH_RADIUS * K_FACTOR) * hr;
    let ht_calc: f64 = 2.0 * (EARTH_RADIUS * K_FACTOR) * ht;

    let root_hr: f64 = (hr_calc).sqrt();
    let root_ht: f64 = (ht_calc).sqrt();

    let answer: f64 = (root_hr + root_ht) / NM_CONVERSION_FACTOR;

    answer
}
