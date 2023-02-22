use super::{metadata::Metadata, step_config::StepConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Step {
    /// The name of the station.
    #[serde(flatten)]
    pub metadata: Metadata,
    /// The configuration of the station.
    pub config: Option<StepConfig>,
    /// The command to run in the container
    pub run: Option<String>,
}
