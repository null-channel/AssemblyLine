use serde::{Deserialize,Serialize};
/// An incoming PushEvent from Github Webhook.

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct PullRequestEvent {
    pub url: String,
    pub id: i64,
    pub number: i64,
}