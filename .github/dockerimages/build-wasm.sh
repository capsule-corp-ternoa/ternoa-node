# Build the binary in release mode
# $HOME/.cargo/bin/cargo build --locked --release

# Install srtool-cli
$HOME/.cargo/bin/cargo install --git https://github.com/chevdor/srtool-cli

# Move the wasm to a new folder called output
mkdir -p output

# Move the ternoa-apllets file so that srtool doesn't complain
mv ./ternoa-pallets/Cargo.toml ./ternoa-pallets/Cargo-copy.toml

# Execute srtool
sudo $HOME/.cargo/bin/srtool build --root --package "$runtime"-runtime &> output/output.txt

cp ./runtime/"$runtime"/target/srtool/release/wbuild/"$runtime"-runtime/"$runtime"_runtime.compact.compressed.wasm ./output/
cp ./runtime/"$runtime"/target/srtool/release/wbuild/"$runtime"-runtime/"$runtime"_runtime.compact.wasm ./output/
cp ./runtime/"$runtime"/target/srtool/release/wbuild/"$runtime"-runtime/"$runtime"_runtime.wasm ./output/

mv ./ternoa-pallets/Cargo-copy.toml ./ternoa-pallets/Cargo.toml

