This directory is used to build the high-level CRI APIs which can talk to CRI implementations like Containerd and CRI-O.

The proto file was taken from the Kubernetes CRI-API repo which itself is a mirror from the main Kubernetes mono repo.

https://github.com/kubernetes/cri-api/blob/master/pkg/apis/runtime/v1/api.proto

Communicating at this level gives us nice features like image pulling from registries

There is a nice summary at https://medium.com/codex/breakdown-kubernetes-container-runtime-2e52bdea5e6
