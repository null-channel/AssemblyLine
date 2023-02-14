use serde::{Serialize, Deserialize};
use super::{
    metadata::Metadata,
    station_config::{StationInput,StationOutput,StationConfig},
    step::Step
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Station {
    /// The name of the station.
    pub metadata: Metadata,
    /// The ids of the stations that this station depends on.
    pub dependencies: Vec<String>,
    /// The input of the station.
    pub input: Option<StationInput>,
    /// The output of the station. not sure if this is needed.
    pub output: Option<StationOutput>,
    /// The configuration of the station.
    pub config: Option<StationConfig>,
    /// The list of steps to run in the station.
    pub steps: Vec<Step>,
}