use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StepConfig {
    pub input: Vec<String>,
    pub output: Vec<String>,
}