set -euo pipefail

case `uname -s` in
    Linux)
        echo "Building static binaries using ekidd/rust-musl-builder"
        docker build -t build-"$1"-image .
        docker run -it --name build-"$1" build-"$1"-image
        docker cp build-"$1":/home/rust/src/target/x86_64-unknown-linux-musl/release/"$1" "$1"
        docker rm build-"$1"
        docker rmi build-"$1"-image
        zip "$1"-"$2".zip "$1"
        ;;
    *)
        echo "Building standard release binaries"
        cargo build --release
	cargo build --target="aarch64-linux-android" --release
	cargo build --target="arm-linux-androideabi" --release 
	zip -j "arm"-"$1"-"$2".zip target/arm-linux-androideabi/release --release
	zip -j "aarch64"-"$1"-"$2".zip target/aarch64-linux-android/release --target
        zip -j "$1"-"$2".zip target/release/"$1"
        ;;
esac
