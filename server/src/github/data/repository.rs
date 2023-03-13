use serde::{Deserialize,Serialize};
/// An incoming PushEvent from Github Webhook.

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Repository {
    pub full_name: String,
    pub private: bool,
}