use std::collections::HashMap;

use cri_lib::{run_container, requests::RunContainerRequest, requests::CleanUp};
use log::info;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    info!("Running the test app for the Assembly Line CRI library");
    let request = RunContainerRequest {
        image: "docker.io/busybox:latest".into(), 
        cmd: vec!["sleep".to_string()], 
        args: vec!["2m".to_string()],
        clean_up: CleanUp::After(12),
        envs: HashMap::new(),
    };

    info!("Running the container");
    run_container(request)
        .await
        .expect("failed to create container");
}
