use super::station_config::{StationConfig, StationInput, StationOutput};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    /// The id of the station.
    pub id: String,
    /// The name of the station.
    /// If none is given the UI can parse the id as it sees fit
    pub name: Option<String>,
    /// The description of the station.
    /// Is optional, with no display if none is given.
    pub description: Option<String>,
}
