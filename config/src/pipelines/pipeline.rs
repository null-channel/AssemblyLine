use serde::{Serialize, Deserialize};
use super:: {
    station::Station,
    events::PipelineEvent,
    metadata::Metadata
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Pipeline {
    pub metadata: Metadata,
    /// stations in the pipeline.
    pub stations: Vec<Station>,
    /// pipeline events to run this pipeline
    pub events: Vec<PipelineEvent>,
    /// remote pipeline to run this pipeline
    pub remote: String,
}