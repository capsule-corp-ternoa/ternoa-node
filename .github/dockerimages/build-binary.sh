# Build the binary in release mode
$HOME/.cargo/bin/cargo build --locked --release

# Move the binary to a new folder called output
mkdir -p output
cp ./target/release/ternoa ./output/

