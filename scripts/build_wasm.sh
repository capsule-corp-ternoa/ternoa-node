RUNTIME=""

while getopts r: flag
do
    case "${flag}" in
        r) RUNTIME=${OPTARG};;
    esac
done

if [ -z "$RUNTIME" ]; then
    select RUNTIME in alphanet mainnet
    do
        break;
    done
fi

cargo build --release -p $RUNTIME-runtime

mkdir -p wasm
cp ./target/release/wbuild/$RUNTIME-runtime/"$RUNTIME"_runtime.compact.wasm ./wasm