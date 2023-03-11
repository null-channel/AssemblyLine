use serde::Deserialize;
/// An incoming PushEvent from Github Webhook.
#[derive(Deserialize)]
struct PushEvent {
    // ref is a reserved keyword in Rust, so we need to rename it.
    #[serde(rename = "ref")]
    reference: String,
}