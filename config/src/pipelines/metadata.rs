use serde::{Serialize, Deserialize};
use super::station_config::{StationInput,StationOutput,StationConfig};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    /// The name of the station.
    pub name: String,
    /// The id of the station.
    /// defaults to the name of the station.
    pub id: String,
    /// The description of the station.
    pub description: String,
}