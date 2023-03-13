use serde::{Deserialize, Serialize};
/// An incoming PushEvent from Github Webhook.

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CheckRunEvent {
    pub name: String,
    pub head_sha: String,
}
