use crate::helpers::InputUnit;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "detect-range",
    version,
    author,
    about,
    after_help = "Operation:\n detect-range     Run in interactive mode\n detect-range --hr <height of radar> --hru <unit m/ft> --ht <height of target> --htu <unit m/ft>      Run in CLI mode"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    // Single Calculation - Returns JSON
    Calc(CalcArgs),
    //Single Graph Calculation - Returns JSON
    Graph(GraphArgs),
}

#[derive(Parser)]
pub struct CalcArgs {
    #[arg(long, value_name = "HEIGHT_RADAR", requires = "hru")]
    /// Height of Radar in Meters (AGL)
    pub hr: f64,

    #[arg(long, value_name = "HEIGHT_RADAR_UNIT")]
    /// Radar Input Unit (AGL)
    pub hru: InputUnit,

    #[arg(long, short = 't', value_name = "HEIGHT_TARGET", requires = "htu")]
    /// Height of Target in feet (AGL)
    pub ht: f64,

    #[arg(long, value_name = "HEIGHT_TARGET_UNIT")]
    /// Target Input Unit
    pub htu: InputUnit,
}

#[derive(Parser)]
pub struct GraphArgs {
    #[arg(long, value_name = "HEIGHT_RADAR", requires = "hru")]
    /// Height of Radar in Meters (AGL)
    pub hr: f64,
    #[arg(long, value_name = "HEIGHT_RADAR_UNIT")]
    /// Radar Input Unit (AGL)
    pub hru: InputUnit,
}
