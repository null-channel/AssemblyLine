use log::{debug, error};
use runtime::v1::{
    image_service_client::ImageServiceClient, runtime_service_client::RuntimeServiceClient,
    CreateContainerRequest, ImageSpec, LinuxPodSandboxConfig, ListContainersRequest,
    ListImagesRequest, Mount, PullImageRequest, RemoveContainerRequest, RemovePodSandboxRequest,
    RunPodSandboxRequest, StartContainerRequest, VersionRequest,
};
use std::collections::HashMap;
use tonic::transport::Channel;

mod runtime {
    pub mod v1 {
        tonic::include_proto!("runtime.v1");
    }
}

mod channel;
pub mod requests;
use channel::*;
use requests::*;

pub async fn run_container(request: RunContainerRequest) -> Result<(), Box<dyn std::error::Error>> {
    let Ok(channel) = create_channel_to_unix_socket().await else {
        error!("Failed to connect to your CRI service - this game is over...");
        return Err("Failed to connect to the CRI service".into())
    };

    let mut client: RuntimeServiceClient<Channel> = RuntimeServiceClient::new(channel.clone());
    let mut image_client: ImageServiceClient<Channel> = ImageServiceClient::new(channel);

    get_version(&mut client, "v1").await?;

    list_images(&mut image_client).await?;

    pull_image(&mut image_client, request.image.clone()).await?;

    create_container(&mut client, request).await?;

    Ok(())
}

async fn get_version(
    client: &mut RuntimeServiceClient<Channel>,
    version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let version_response = client
        .version(VersionRequest {
            version: version.into(),
        })
        .await?;

    debug!("Version is {version_response:?}");
    Ok(())
}

async fn list_images(
    image_client: &mut ImageServiceClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    let image_list_response = image_client
        .list_images(ListImagesRequest { filter: None })
        .await?;
    // .expect("Failed to list images");

    debug!("IMAGES: {image_list_response:?}");
    Ok(())
}

async fn pull_image(
    image_client: &mut ImageServiceClient<Channel>,
    image: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let image_pull_response = image_client
        .pull_image(PullImageRequest {
            image: Some(ImageSpec {
                image,
                ..Default::default()
            }),
            auth: None,
            sandbox_config: None,
        })
        .await?;
    // .expect("Failed to pull image");

    debug!("PULLED IMAGE: {image_pull_response:?}");
    Ok(())
}

async fn create_container(
    client: &mut RuntimeServiceClient<Channel>,
    request: RunContainerRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let pod_sandbox_config = runtime::v1::PodSandboxConfig {
        metadata: Some(runtime::v1::PodSandboxMetadata {
            name: "busybox_test".into(),
            uid: "x4k9sh7fsb3e9s89sdd7238e".into(),
            namespace: "assemblyline".into(),
            attempt: 1,
        }),
        hostname: "al_test".into(),
        log_directory: "/tmp".into(),
        dns_config: None,
        port_mappings: vec![],
        labels: HashMap::new(),
        annotations: HashMap::new(),
        linux: Some(LinuxPodSandboxConfig::default()),
        windows: None,
    };

    let run_pod_sandbox_request = RunPodSandboxRequest {
        config: Some(pod_sandbox_config.clone()),
        // https://github.com/kubernetes/enhancements/tree/master/keps/sig-node/585-runtime-class#runtime-handler
        // empty string = default, think we could specify "runc" "gVisor" etc.
        runtime_handler: "".into(),
    };

    let mut container_id: String = "".into();
    let should_create_container = {
        let list_containers_response = client
            .list_containers(ListContainersRequest { filter: None })
            .await?;
        debug!("LIST CONTAINERS: {list_containers_response:?}");
        let container_exists = list_containers_response
            .get_ref()
            .containers
            .iter()
            .any(|x| x.image.is_some() && x.image.as_ref().unwrap().image == request.image);

        if container_exists {
            container_id = list_containers_response
                .get_ref()
                .containers
                .iter()
                .find(|x| x.image.is_some() && x.image.as_ref().unwrap().image == request.image)
                .unwrap()
                .id
                .clone();
        }

        !container_exists
    };

    let mut pod_sandbox_id: String = "".into();
    if should_create_container {
        let run_pod_response = client.run_pod_sandbox(run_pod_sandbox_request).await?;
        debug!("RUN POD SANDBOX: {run_pod_response:?}");

        pod_sandbox_id = run_pod_response.get_ref().pod_sandbox_id.clone();

        let create_container_response = client
            .create_container(CreateContainerRequest {
                config: Some(runtime::v1::ContainerConfig {
                    metadata: Some(runtime::v1::ContainerMetadata {
                        name: "altest1".into(),
                        attempt: 0,
                    }),
                    image: Some(ImageSpec {
                        image: request.image.clone(),
                        annotations: HashMap::new(),
                    }),
                    command: request.cmd.clone(),
                    args: request.args.clone(),
                    working_dir: "/".into(),
                    envs: request.envs_as_vec(),
                    mounts: vec![
                        Mount {
                            container_path: "/output/".into(),
                            host_path: "/tmp/output/".into(),
                            readonly: false,
                            selinux_relabel: false,
                            /* enum MountPropagation {
                                PROPAGATION_PRIVATE = 0;
                                PROPAGATION_HOST_TO_CONTAINER = 1;
                                PROPAGATION_BIDIRECTIONAL = 2;
                            }*/
                            propagation: 0,
                        },
                        Mount {
                            container_path: "/input/".into(),
                            host_path: "/tmp/input/".into(),
                            readonly: true,
                            selinux_relabel: false,
                            propagation: 0,
                        },
                    ],
                    devices: vec![],
                    labels: HashMap::new(),
                    annotations: HashMap::new(),
                    log_path: "/tmp/busybox.log".into(),
                    stdin: true,
                    stdin_once: false,
                    tty: true,
                    linux: Some(runtime::v1::LinuxContainerConfig::default()),
                    windows: None,
                }),
                pod_sandbox_id: pod_sandbox_id.clone(),
                sandbox_config: Some(pod_sandbox_config),
            })
            .await?;

        debug!("CREATE CONTAINER: {create_container_response:?}");

        container_id = create_container_response.get_ref().container_id.clone();
    }

    let start_container_response = client
        .start_container(StartContainerRequest {
            container_id: container_id.clone(),
        })
        .await?;
    debug!("START CONTAINER: {start_container_response:?}");

    match request.clean_up {
        CleanUp::Never => return Ok(()),
        CleanUp::After(seconds) => std::thread::sleep(std::time::Duration::from_secs(seconds)),
        CleanUp::Immediate => (),
    }

    let _remove_container_response = client
        .remove_container(RemoveContainerRequest {
            container_id: container_id.clone(),
        })
        .await?;

    let _remove_pod_sandbox_response = client
        .remove_pod_sandbox(RemovePodSandboxRequest { pod_sandbox_id })
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {}
}
