use serde::{Deserialize,Serialize};
/// An incoming PushEvent from Github Webhook.

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Sender {
    pub login: String,
    pub url: String,
}