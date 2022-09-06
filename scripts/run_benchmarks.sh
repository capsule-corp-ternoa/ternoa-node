#!/bin/bash

# Default vaules
DURATION="short"
CHAIN="alphanet-dev"
PALLET="*"
OUTPUT="./weights/"

# Read flags
while getopts d:c:p:o: flag
do
    case "${flag}" in
        d) DURATION=${OPTARG};;
        c) CHAIN=${OPTARG};;
        p) PALLET=${OPTARG};;
        p) OUTPUT=${OPTARG};;
    esac
done

# echo "Building the Ternoa client..."
# cargo build --release --features runtime-benchmarks

if [ "$DURATION" = "long" ]; then
    COMMAND="./target/release/ternoa benchmark pallet --steps=50 --repeat=20 --extrinsic=* --execution=wasm --wasm-execution=compiled --heap-pages=4096"
elif [ "$DURATION" = "medium" ]; then
    COMMAND="./target/release/ternoa benchmark pallet --steps=10 --repeat=5 --extrinsic=* --execution=wasm --wasm-execution=compiled --heap-pages=4096"
elif [ "$DURATION" = "short" ]; then
    COMMAND="./target/release/ternoa benchmark pallet --steps=2 --repeat=1 --extrinsic=* --execution=wasm --wasm-execution=compiled --heap-pages=4096"
else 
    echo "Unknown duration. Supported value: long; medium; short"
    exit 0;
fi

if [ "$OUTPUT" = "./weights/" ]; then
    mkdir -p weights
fi

COMMAND="$COMMAND --pallet=$PALLET --chain $CHAIN --output=$OUTPUT"

echo "Duration: $DURATION"
echo "Chain: $CHAIN"
echo "Pallet: $PALLET"
echo "Output: $OUTPUT"
echo "Command: $COMMAND"

echo $(eval $COMMAND)