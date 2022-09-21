#!/bin/bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
REPO_DIR=$SCRIPT_DIR/..

# Read flags
while getopts f: flag
do
    case "${flag}" in
        f) flavour=${OPTARG};;
    esac
done

if [ -z "$flavour" ]; then
    select flavour in ubuntu-2004 ubuntu-2204 fedora-35 fedora-36 debian-11 arch
    do
        break;
    done
fi

# Get Image name
if [ "$flavour" = "ubutnu-2004" ]; then
    imageName="ubuntu-2004.Dockerfile"
elif [ "$flavour" = "ubuntu-2204" ]; then
    imageName="ubuntu-2204.Dockerfile"
elif [ "$flavour" = "fedora-35" ]; then
    imageName="fedora-35.Dockerfile"
elif [ "$flavour" = "fedora-36" ]; then
    imageName="fedora-36.Dockerfile"
elif [ "$flavour" = "debian-11" ]; then
    imageName="debian-11.Dockerfile"
elif [ "$flavour" = "arch" ]; then
    imageName="arch.Dockerfile"
else
    echo "Unknown option."
    exit 0
fi

echo "Selected flavour: $flavour"

# Move the right directory
cd $REPO_DIR

# Build the image
podman build -t tchain -f ./dockerimages/$imageName .

# Run the image
mkdir -p output/$flavour
podman run --rm -v ./output/$flavour:/output tchain
