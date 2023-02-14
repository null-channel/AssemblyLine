use serde::{Serialize, Deserialize};
use super::events::{PipelineEvent};

/// A pipeline config.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineConfig {

}

/// The input of a pipeline.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineInput {

}

/// The output of a pipeline. Not sure if we need this.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineOutput {

}