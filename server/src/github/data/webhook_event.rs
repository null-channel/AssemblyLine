use serde::{Deserialize, Serialize};

use super::{
    check_run::CheckRunEvent, organization::Organization, pull_request::PullRequestEvent,
    repository::Repository, sender::Sender,
};
/// An incoming WebHookEvent from Github Webhook.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WebhookEvent {
    pub action: String,

    // The following fields are optional because they are only present if that
    // event type is sent.
    pub pull_request: Option<PullRequestEvent>,
    pub check_run: Option<CheckRunEvent>,

    // These fields are always present. and contain general information about
    // the repository and the sender of the event
    pub repository: Option<Repository>,
    pub organization: Option<Organization>,
    pub sender: Option<Sender>,
}
