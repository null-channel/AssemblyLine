const PROTO_FILES: &[&str] = &[
    "vendor/github.com/kubernetes/cri-api/pkg/apis/runtime/v1/api.proto",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {

    tonic_build::configure()
        .build_server(true)
        .compile(PROTO_FILES, &["vendor/"])
        .expect("Failed to generate GRPC bindings");

    Ok(())
}