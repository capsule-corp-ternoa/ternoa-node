# Build the binary in release mode
$HOME/.cargo/bin/cargo install --git https://github.com/chevdor/srtool-cli

# Move the wasm to a new folder called output
mkdir -p output
srtool build --package "$Runtime"-runtime &> output/output.txt

cp ./target/release/ternoa ./output/
cp ./runtime/"$Runtime"/target/srtool/release/wbuild/"$Runtime"-runtime/"$Runtime"_runtime.compact.compressed.wasm ./output/
cp ./runtime/"$Runtime"/target/srtool/release/wbuild/"$Runtime"-runtime/"$Runtime"_runtime.compact.wasm ./output/
cp ./runtime/"$Runtime"/target/srtool/release/wbuild/"$Runtime"-runtime/"$Runtime"_runtime.wasm ./output/

