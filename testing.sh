export Runtime="alphanet"

# podman build -t tchain -f ./.github/dockerimages/dev-ubuntu-22.Dockerfile .
# podman run --rm -v ./.:/workdir -t tchain /bin/bash .github/dockerimages/build-binary.sh
# podman run --rm -v ./.:/workdir -t tchain /bin/bash .github/dockerimages/build-binary.sh
# podman run -e Runtime --rm -v ./.:/workdir -t tchain /bin/bash .github/dockerimages/run-benchmarks.sh
# podman run -e Runtime --rm -v ./.:/workdir -t tchain /bin/bash .github/dockerimages/run-format.sh
podman run --rm -v ./output:/workdir/output -t tchain /bin/bash .github/dockerimages/build-binary.sh
