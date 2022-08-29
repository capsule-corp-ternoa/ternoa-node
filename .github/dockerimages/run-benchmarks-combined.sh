# Build the binary in release mode
$HOME/.cargo/bin/cargo build --locked --release --features runtime-benchmarks

./scripts/benchmarks/"$Runtime"/run_benchmarks.sh

# Move the weights to a new folder called output
cp ./weights/* ./output