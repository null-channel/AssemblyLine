use serde::{Serialize, Deserialize};

/// An event that can triger a pipeline to run
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PipelineEvent {
    On(OnAction),
    Webhook(WebhookAction),
    Manual(ManualAction),
}

/// Think GHA "on" pr/merge/ect
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OnAction {
    pub action_id: String,
 
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