#podman build -t tchain -f ./.github/dockerimages/dev-ubuntu-22.Dockerfile .
podman run --rm -v ./.:/workdir -t tchain /bin/bash .github/dockerimages/build-binary.sh
