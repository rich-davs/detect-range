use serde::Serialize;

use crate::helpers::*;
use crate::radar_horizon;

#[derive(Debug, Serialize)]
pub struct GraphPoint {
    pub range_m: f64,
    pub alt_m: f64,
}

#[derive(Debug, Serialize)]
pub struct PlotPoint {
    pub range_nm: f64,
    pub alt_ft: f64,
}

pub fn graph_meters(hrm: f64) -> Vec<GraphPoint> {
    let mut results = Vec::new();

    for alt_i in (0..=20000).step_by(25) {
        let alt_m = alt_i as f64;
        let range_m = radar_horizon(hrm, alt_m);
        let point = GraphPoint { range_m, alt_m };
        results.push(point);
    }
    results
}

pub fn graph_ft(hrm: f64) -> Vec<PlotPoint> {
    let points: Vec<GraphPoint> = graph_meters(hrm);
    let plot_points: Vec<PlotPoint> = points
        .iter()
        .map(|i| PlotPoint {
            range_nm: i.range_m / METERS_TO_NM,
            alt_ft: i.alt_m / METERS_TO_FT,
        })
        .collect();
    plot_points
}
