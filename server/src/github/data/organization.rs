use serde::{Deserialize, Serialize};
/// An incoming Organization from Github Webhook.

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Organization {
    pub login: String,
}
