use serde::{Deserialize, Serialize};

/// An event that can triger a pipeline to run
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineEvent {
    pub on: Option<OnAction>,
    pub webhook: Option<WebhookAction>,
    pub manual: Option<ManualAction>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OnAction {
    pub pr: Option<OnPullRequest>,
    pub push: Option<OnPush>,
    pub schedule: Option<OnSchedule>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OnPullRequest {
    pub branches: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OnPush {
    branches: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OnSchedule {
    cron: String,
}

/// Think external system integration
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookAction {
    pub action_id: String,
}

/// Think "I want to run this manually"
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualAction {
    pub action_id: String,
}
