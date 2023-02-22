use serde::{Deserialize, Serialize};
pub mod events;
pub mod metadata;
pub mod station;
pub mod station_config;
pub mod step;
pub mod step_config;

use events::PipelineEvent;
use metadata::Metadata;
use station::Station;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Pipeline {
    #[serde(flatten)]
    pub metadata: Metadata,
    /// stations in the pipeline.
    pub stations: Vec<Station>,
    /// pipeline events to run this pipeline
    pub events: Vec<PipelineEvent>,
}

impl Validate for Pipeline {
    fn validate(&self) -> Result<(), String> {
        if self.events.len() == 0 {
            return Err("Pipeline must have at least one event".to_string());
        }

        if self.stations.len() == 0 {
            return Err("Pipeline must have at least one station".to_string());
        }

        if self.metadata.id == "" {
            return Err("Pipeline must have an id".to_string());
        }

        Ok(())
    }
}

pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}
