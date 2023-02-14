use serde::{Serialize, Deserialize};
use super::{
    metadata::Metadata,
    step_config::StepConfig
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Step {
    /// The name of the station.
    pub metadata: Metadata,
    /// The configuration of the station.
    pub config: Option<StepConfig>,
    /// The command to run in the container
    pub run: String,
}
