# Build the binary in production mode
$HOME/.cargo/bin/cargo build --profile production --locked --features=runtime-benchmarks

# Create the output folder if it doesn't exist
mkdir -p output

# Run our run_benchmark script
./scripts/run_benchmarks.sh -o ./output -r "$runtime"