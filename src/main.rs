// SPDX-License-Identifier: MIT OR Apache-2.0

//! detect-range
//! Author: Rich Davs
//!
//! Radar Detection line-of-sight calculator utilising the
//! effective Earth radius / k-factor model commonly used in Radar Engineering.

mod cli;
mod graph;
mod helpers;
use crate::cli::*;
use crate::graph::*;
use crate::helpers::*;
use clap::Parser;

use serde_json::json;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Calc(calc) => {
            let hrm = resolve_input_m(calc.hr, calc.hru);
            let htm = resolve_input_m(calc.ht, calc.htu);
            let result_meters = radar_horizon(hrm, htm);
            let output: OutputUnit = unit_converter(result_meters);
            calc_json_output(calc.hr, calc.hru, calc.ht, calc.htu, hrm, htm, output);
        }
        Commands::Graph(graph) => {
            let hrm = resolve_input_m(graph.hr, graph.hru);
            let plot_points: Vec<PlotPoint> = graph_ft(hrm);
            graph_json_output(graph.hr, graph.hru, plot_points)
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

fn calc_json_output(
    hr: f64,
    hru: InputUnit,
    ht: f64,
    htu: InputUnit,
    hrm: f64,
    htm: f64,
    output: OutputUnit,
) {
    let payload = json!({
    "input": {
        "radar_height": { "value": hr, "unit": hru },
        "target_height": { "value": ht, "unit": htu },
        "normalized" : { "radar_height_m": hrm, "target_height_m": htm},
    },
    "result": {
        "detection_range": output,
    },
    });
    println!("{}", serde_json::to_string_pretty(&payload).unwrap());
}

fn graph_json_output(hr: f64, hru: InputUnit, plot_points: Vec<PlotPoint>) {
    let payload = json!({
    "input": {
        "radar_height": { "value": hr, "unit": hru },
    },
    "graph_data": plot_points,
    });
    println!("{}", serde_json::to_string_pretty(&payload).unwrap());
}
