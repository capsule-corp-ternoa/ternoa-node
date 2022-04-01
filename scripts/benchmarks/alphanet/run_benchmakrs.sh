echo "Building the Ternoa client..."
# cargo build --release --features runtime-benchmarks

COMMAND="./target/release/ternoa benchmark --chain alphanet-dev --steps=50 --repeat=20 --extrinsic=* --execution=wasm --wasm-execution=compiled --heap-pages=4096 --output=./runtime/alphanet/src/weights/"
MEDIUM_COMMAND="./target/release/ternoa benchmark --chain alphanet-dev --steps=10 --repeat=5 --extrinsic=* --execution=wasm --wasm-execution=compiled --heap-pages=4096 --output=./runtime/alphanet/src/weights/"
SHORT_COMMAND="./target/release/ternoa benchmark --chain alphanet-dev --steps=5 --repeat=2 --extrinsic=* --execution=wasm --wasm-execution=compiled --heap-pages=4096 --output=./runtime/alphanet/src/weights/"

echo "Running non-ternoa pallet benchmarks"

echo "PALLET BABE"
echo $(eval $COMMAND --pallet=pallet_babe)

echo "PALLET BAGS LIST"
echo $(eval $COMMAND --pallet=pallet_bags_list)

echo "PALLET BALANCES"
echo $(eval $COMMAND --pallet=pallet_balances)

echo "PALLET ELECTION PROVIDER MULTI PHASE"
echo $(eval $SHORT_COMMAND --pallet=pallet_election_provider_multi_phase)

echo "PALLET GRANDPA"
echo $(eval $COMMAND --pallet=pallet_grandpa)

echo "PALLET IM ONLINE" 
echo $(eval $SHORT_COMMAND --pallet=pallet_im_online)

echo "PALLET MEMBERSHIP"
echo $(eval $COMMAND --pallet=pallet_membership)

echo "PALLET PREIMAGE"
echo $(eval $COMMAND --pallet=pallet_preimage)

echo "PALLET SCHEDULER"
echo $(eval $COMMAND --pallet=pallet_scheduler)

# echo "TODO PALLET SESSION"
# echo $(eval $COMMAND --pallet=pallet_session)

# echo "TODO PALLET STAKING"
# echo $(eval $COMMAND --pallet=pallet_staking)

echo "PALLET TIMESTAMP"
echo $(eval $COMMAND --pallet=pallet_timestamp)

echo "PALLET TREASURY"
echo $(eval $COMMAND --pallet=pallet_treasury)

echo "PALLET UTILITY"
echo $(eval $COMMAND --pallet=pallet_utility)

echo "PALLET COLLECTIVE"
echo $(eval $MEDIUM_COMMAND --pallet=pallet_collective)

echo "PALLET COLLECTIVE"
echo $(eval $COMMAND --pallet=frame_system)

echo "Running Ternoa pallet benchmarks"

echo "TERNOA STAKING REWARDS"
echo $(eval $COMMAND --pallet=ternoa_staking_rewards)