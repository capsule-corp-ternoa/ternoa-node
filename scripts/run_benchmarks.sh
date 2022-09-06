#!/bin/bash

# Default vaules
DURATION="long"
RUNTIME="alphanet"
PALLET="*"
OUTPUT_FOLDER="./weights/"
STEPS=50
REPEAT=20
DEV=false
PROD=false

# Read flags
while getopts d:r:p:o:dev:prod: flag
do
    case "${flag}" in
        d) DURATION=${OPTARG};;
        r) RUNTIME=${OPTARG};;
        p) PALLET=${OPTARG};;
        o) OUTPUT_FOLDER=${OPTARG};;
        dev) DEV=true;;
        prod) PROD=true;;
    esac
done

if $DEV; then
    DURATION="short"
fi

if $PROD; then
    DURATION="long"
fi

# echo "Building the Ternoa client..."
# cargo build --profile production --locked --features=runtime-benchmarks

CHAIN="$RUNTIME-dev"

if [ "$DURATION" = "long" ]; then
    STEPS=50
    REPEAT=20
elif [ "$DURATION" = "medium" ]; then
    STEPS=10
    REPEAT=5
elif [ "$DURATION" = "short" ]; then
    STEPS=2
    REPEAT=1
else 
    echo "Unknown duration. Supported value: long; medium; short"
    exit 0;
fi

if [ "$OUTPUT" = "./weights/" ]; then
    mkdir -p weights
fi

echo "Duration: $DURATION"
echo "Chain: $CHAIN"
echo "Pallet: $PALLET"
echo "Output folder: $OUTPUT_FOLDER"
echo "Steps: $STEPS"
echo "Repeat: $REPEAT"


# Manually exclude some pallets.
EXCLUDED_PALLETS=(
  # Helper pallets
  "pallet_election_provider_support_benchmarking"
  # Pallets without automatic benchmarking
  "pallet_babe"
  "pallet_grandpa"
  "pallet_mmr"
  "pallet_offences"
)

if [ "$PALLET" = "*" ]; then
    PALLETS=($(./target/production/ternoa benchmark pallet --list --chain $CHAIN | tail -n+2 | cut -d',' -f1 | sort | uniq ))
else
    PALLETS=($PALLET)
fi

if [ "$OUTPUT_FOLDER" = "./weights/" ]; then
    mkdir -p weights
fi

ERR_FILE="benchmarking_errors.txt"
# Delete the error file before each run.
rm -f $ERR_FILE

# Benchmark each pallet.
for PALLET in "${PALLETS[@]}"; do
    SKIP=false
    for EXCLUDED_PALLET in "${EXCLUDED_PALLETS[@]}"; do
        if [ "$EXCLUDED_PALLET" == "$PALLET" ]; then
            SKIP=true
            break
        fi
    done

    if $SKIP; then
        echo "[ ] Skipping pallet $PALLET";
        continue
    fi

    echo "[+] Benchmarking $PALLET";

    OUTPUT=$(./target/production/ternoa benchmark pallet --chain=$CHAIN --steps=$STEPS --repeat=$REPEAT --pallet="$PALLET" --extrinsic="*" --execution=wasm --wasm-execution=compiled --heap-pages=4096 --output $OUTPUT_FOLDER 2>&1 )
    if [ $? -ne 0 ]; then
        echo "$OUTPUT" >> "$ERR_FILE"
        echo "[-] Failed to benchmark $PALLET. Error written to $ERR_FILE; continuing..."
    fi
done
