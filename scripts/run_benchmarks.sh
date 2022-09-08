#!/bin/bash

# Default vaules
QUICK_EXECUTION=false
RUNTIME="alphanet"
STEPS=50
REPEAT=20
OUTPUT_FOLDER="./weights"
PALLET="*"
START_TIMER_1=$(date +%s)
BUILD_BINARY=false

# Read flags
while getopts qr:p:o:b flag
do
    case "${flag}" in
        q) QUICK_EXECUTION=true;;
        r) RUNTIME=${OPTARG};;
        p) PALLET=${OPTARG};;
        o) OUTPUT_FOLDER=${OPTARG};;
        b) BUILD_BINARY=true;;
    esac
done

CHAIN="$RUNTIME-dev"
if $QUICK_EXECUTION; then
    STEPS=2
    REPEAT=1
fi

echo "Chain: $CHAIN"
echo "Output folder: $OUTPUT_FOLDER"
echo "Steps: $STEPS"
echo "Repeat: $REPEAT"
echo "Pallet: $PALLET"
echo "Build Binary: $BUILD_BINARY"

START_TIMER_2=$(date +%s)
if "$BUILD_BINARY"; then
    echo "Building the Ternoa client..."
    cargo build --profile production --locked --features=runtime-benchmarks
fi
END_TIMER_2=$(date +%s)


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

if [ "$OUTPUT_FOLDER" = "./weights" ]; then
    mkdir -p weights
fi



ERR_FILE="$OUTPUT_FOLDER/benchmarking_errors.txt"
# Delete the error file before each run.
rm -f $ERR_FILE

START_TIMER_3=$(date +%s)
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
END_TIMER_3=$(date +%s)
END_TIMER_1=$(date +%s)



secs=$(($END_TIMER_1-$START_TIMER_1))
printf 'Total Elapsed Time: %02dh:%02dm:%02ds\n' $((secs/3600)) $((secs%3600/60)) $((secs%60))
secs=$(($END_TIMER_2-$START_TIMER_2))
printf 'Binary Build Elapsed Time: %02dh:%02dm:%02ds\n' $((secs/3600)) $((secs%3600/60)) $((secs%60))
secs=$(($END_TIMER_3-$START_TIMER_3))
printf 'Benchmark Exeuction Elapsed Time: %02dh:%02dm:%02ds\n' $((secs/3600)) $((secs%3600/60)) $((secs%60))