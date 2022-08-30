export runtime="alphanet"

podman build -t tchain -f ./dockerimages/ubuntu-2204-dev.Dockerfile .
# podman run --rm -v ./.:/workdir -t tchain /bin/bash .github/dockerimages/build-binary.sh
# podman run --rm -v ./.:/workdir -t tchain /bin/bash .github/dockerimages/build-binary.sh
# podman run -e Runtime --rm -v ./.:/workdir -t tchain /bin/bash .github/dockerimages/run-benchmarks.sh
# podman run -e Runtime --rm -v ./.:/workdir -t tchain /bin/bash .github/dockerimages/run-format.sh
podman run --rm -v ./output:/output -e runtime=$runtime -t tchain /bin/bash .github/dockerimages/build-wasm.sh
